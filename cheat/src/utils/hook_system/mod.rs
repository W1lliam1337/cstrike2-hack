use crate::common;
use common::*;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Hook {
    target: Arc<Mutex<*mut c_void>>,
    detour: Arc<Mutex<*mut c_void>>,
    original: Arc<Mutex<*mut c_void>>,
}

lazy_static::lazy_static! {
    static ref TARGETS: Mutex<VecDeque<Hook>> = Mutex::new(VecDeque::new());
}

unsafe impl Send for Hook {}

impl Hook {
    pub fn get_proto_original<F, R>(func: F) -> Option<R>
    where
        F: Fn() -> *mut c_void,
        R: From<*mut c_void>,
    {
        let targets = TARGETS.lock().unwrap();
        let it = targets.iter().find(|hook| *hook.detour.lock().unwrap() == func());
        it.map(|hook| R::from(*hook.original.lock().unwrap()))
    }

    pub fn hook(target: *const c_void, detour: *const c_void) -> bool {
        let mut targets = TARGETS.lock().unwrap();
        let h = Hook {
            target: Arc::new(Mutex::new(target as *mut c_void)),
            detour: Arc::new(Mutex::new(detour as *mut c_void)),
            original: Arc::new(Mutex::new(std::ptr::null_mut())),
        };

        unsafe {
            if minhook_sys::MH_CreateHook(
                *h.target.lock().unwrap(),
                *h.detour.lock().unwrap(),
                &mut *h.original.lock().unwrap() as *mut *mut c_void,
            ) == 0
            {
                minhook_sys::MH_EnableHook(*h.target.lock().unwrap());
                targets.push_back(h);
                true
            } else {
                false
            }
        }
    }
}

/// Initializes the MinHook library.
///
/// This function initializes the MinHook library, which is used for creating and managing hooks.
///
/// # Returns
///
/// * `Ok(())` if the MinHook library is successfully initialized.
/// * `Err(String)` if an error occurs during initialization. The error message will provide details about the failure.
pub fn initialize_minhook() -> anyhow::Result<(), String> {
    unsafe {
        if minhook_sys::MH_Initialize() != 0 {
            return Err("Failed to initialize MinHook".to_owned());
        }

        println!("MinHook initialized successfully");
    }

    Ok(())
}

/// This macro is used to create a new hook for a specified target function and detour function.
///
/// # Parameters
///
/// * `$target_function:ident` - The identifier of the target function to be hooked.
/// * `$detour_function:ident` - The identifier of the detour function that will replace the target function.
///
/// # Details
///
/// The macro takes two identifiers as input: `$target_function` and `$detour_function`.
/// It then converts the function pointers to `*mut std::os::raw::c_void` and creates a new `Hook` instance using the `hook_system::Hook::new` function.
/// If the hook creation is successful, it enables the hook using the `hook.enable()` method.
/// If an error occurs during hook creation or enabling, it prints an error message and returns early.
#[macro_export]
macro_rules! create_hook {
    ($target_function:ident, $detour_function:ident) => {
        let target_function_ptr = $target_function.unwrap_or(0) as *mut std::ffi::c_void;
        let detour_function_ptr = $detour_function as *const std::ffi::c_void;

        if target_function_ptr == std::ptr::null_mut() {
            bail!("Target function pointer is null");
        }

        println!("Hooking target function: 0x{:x}", $target_function.unwrap_or(0));

        if !hook_system::Hook::hook(target_function_ptr, detour_function_ptr) {
            bail!("Failed to enable hook");
        }
    };
}

/// This macro is used to generate a function that retrieves the original function pointer of a hooked function.
///
/// # Parameters
///
/// * `$hook_name:ident` - The identifier of the hook to retrieve the original function pointer from.
/// * `$fn_name:ident` - The identifier of the generated function that will hold the original function pointer.
/// * `($($arg:ty),*)` - The types and names of the function's parameters.
/// * `$ret:ty` - The return type of the function.
///
/// # Return
///
/// The macro generates a function named `$fn_name` that takes the same parameters and return type as the original function.
/// This function retrieves the original function pointer from the specified hook and transmutes it to the appropriate function type.
/// The original function pointer is then returned.
#[macro_export]
macro_rules! get_original_fn {
    ($hook_name:ident, $fn_name:ident, ($($arg:ty),*), $ret:ty) => {
        let $fn_name: extern "system" fn($($arg),*) -> $ret = unsafe {
            std::mem::transmute::<
                *mut std::ffi::c_void,
                extern "system" fn($($arg),*) -> $ret,
            >(hook_system::Hook::get_proto_original(|| $hook_name as *mut std::ffi::c_void).unwrap())
        };
    };
}
