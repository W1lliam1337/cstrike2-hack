use crate::common;
use common::{c_void, from_mut, null_mut};

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Represents a function hook.
pub struct Hook {
    /// A pointer to the target function to be hooked.
    target: *mut c_void,
    /// A pointer to the detour function.
    detour: *mut c_void,
    /// A pointer to the original function.
    original: *mut c_void,
}

lazy_static::lazy_static! {
    static ref TARGETS: Arc<Mutex<VecDeque<Hook>>> = Arc::new(Mutex::new(VecDeque::new()));
}

unsafe impl Send for Hook {}

impl Hook {
    /// Retrieves the original function pointer for a given detour function.
    ///
    /// # Parameters
    ///
    /// - `func`: A function that returns the detour function pointer.
    ///
    /// # Returns
    ///
    /// An optional original function pointer wrapped in `Option<R>`.
    ///
    /// # Panics
    ///
    /// This function will panic if the `TARGETS` mutex is poisoned when locked. This might occur
    /// if another thread panics while holding the lock, which is an exceptional case in normal use.
    ///
    /// # Errors
    ///
    /// No errors are returned by this function, but note that the presence of `None` in the return type
    /// indicates that the original function was not found.
    #[inline]
    pub fn get_proto_original<F, R>(func: F) -> Option<R>
    where
        F: Fn() -> *mut c_void,
        R: From<*mut c_void>,
    {
        // Acquire the lock and use the guard directly

        let targets = match TARGETS.lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Failed to lock TARGETS");
                return None;
            }
        };

        // Use the guard to perform the search
        targets.iter().find(|hook| hook.detour == func()).map(|hook| R::from(hook.original))
    }

    /// Hooks a target function with a detour function.
    ///
    /// # Parameters
    ///
    /// - `target`: A pointer to the target function.
    /// - `detour`: A pointer to the detour function.
    ///
    /// # Returns
    ///
    /// `true` if the hook was successfully created and enabled, `false` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if it fails to lock the `TARGETS` mutex.
    #[inline]
    #[must_use]
    pub fn hook(target: *const c_void, detour: *const c_void) -> bool {
        let Ok(mut targets) = TARGETS.lock() else {
            eprintln!("Failed to lock TARGETS");
            return false;
        };

        let mut hk =
            Self { target: target.cast_mut(), detour: detour.cast_mut(), original: null_mut() };

        // SAFETY: Creating the hook with MinHook library.
        let create_hook_result =
            unsafe { minhook_sys::MH_CreateHook(hk.target, hk.detour, from_mut(&mut hk.original)) };

        if create_hook_result == 0 {
            // SAFETY: Enabling the hook with MinHook library.
            unsafe {
                minhook_sys::MH_EnableHook(hk.target);
            };
            targets.push_back(hk);
            true
        } else {
            false
        }
    }
}

/// Initializes the `MinHook` library.
///
/// # Returns
///
/// Returns an `anyhow::Result` indicating success or failure. On success, it returns `Ok(())`. On failure, it returns an `Err` with a description of the error.
///
/// # Errors
///
/// - Returns an `Err` with a description if `MinHook` fails to initialize.
///
/// # Panics
///
/// This function does not panic, but it relies on `minhook_sys::MH_Initialize`, which may potentially fail.
#[inline]
pub fn initialize_minhook() -> anyhow::Result<(), String> {
    // Safety: We are calling an external C library function that initializes MinHook.
    // The function `MH_Initialize` is expected to return 0 on success and a non-zero value on failure.
    // We assume the library's documentation and contract are correct, and we handle the error accordingly.
    unsafe {
        if minhook_sys::MH_Initialize() != 0i32 {
            return Err("Failed to initialize MinHook".to_owned());
        }

        println!("MinHook initialized successfully");
    };

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
        let target_function_ptr = match $target_function {
            Some(func) => func as *mut std::ffi::c_void,
            None => {
                bail!("Target function pointer is null");
            }
        };

        let detour_function_ptr = $detour_function as *const std::ffi::c_void;

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
        // Safety: The `hook_system::Hook::get_proto_original` function is assumed to return a valid function pointer
        // for the specified hook. The `transmute` operation is safe here because the pointer is expected to be valid
        // and the type of the function signature matches the expected type.
        // The correctness of this operation depends on the implementation of `Hook::get_proto_original` and
        // the assumption that the function pointer returned is correctly typed and valid.
        let $fn_name: extern "system" fn($($arg),*) -> $ret = unsafe {
            std::mem::transmute::<
                *mut std::ffi::c_void,
                extern "system" fn($($arg),*) -> $ret,
            >(hook_system::Hook::get_proto_original(|| $hook_name as *mut std::ffi::c_void).unwrap())
        };
    };
}
