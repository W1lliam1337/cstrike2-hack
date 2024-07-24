use crate::{common, utils::module_handler};
use common::*;

use windows::Win32::Foundation::HMODULE;

pub struct Module {
    name: &'static str,
    handle: HMODULE,
}

impl Module {
    pub fn new(name: &'static str) -> Self {
        let handle = module_handler::get_module_handle(name).expect("Failed to get module handle");
        Module { name, handle }
    }

    pub fn find_seq_of_bytes(&self, pattern: &str) -> Option<usize> {
        module_handler::pattern_search(self.handle, pattern)
    }

    pub fn get_export(&self, function_name: &str) -> Option<*mut c_void> {
        module_handler::get_proc_address(self.handle, function_name)
    }

    pub fn get_interface(&self, interface_name: &str) -> Option<*const usize> {
        module_handler::get_interface(self.handle, interface_name)
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

static mut MODULES: Option<Vec<Module>> = None;
static INIT: Once = Once::new();

/// Initializes statically stored `Module` instances.
///
/// This function initializes a vector of `Module` instances using the provided module names.
/// The `Module` instances are stored in a statically allocated `MODULES` vector.
/// The function is called once using the `Once` instance `INIT`.
///
/// # Parameters
///
/// - `names`: A slice of static string references representing module names.
///
/// # Return
///
/// - This function does not return a value.
///
/// # Safety
///
/// The function assumes that the `INIT` instance has not been called before.
/// If the `INIT` instance has already been called, calling this function will result in a panic.
///
/// # Panics
///
/// This function panics if the `INIT` instance has already been called.
pub fn initialize_modules(names: &[&'static str]) {
    INIT.call_once(|| unsafe {
        MODULES = Some(names.iter().map(|&name| Module::new(name)).collect());

        match MODULES.as_ref() {
            Some(modules) => {
                modules.iter().for_each(|module| {
                    println!(
                        "Initialized module: {} {:p}",
                        module.name, module.handle.0 as *const c_void
                    );
                });
            }
            None => {
                eprintln!("Failed to initialize modules");
            }
        }
    });
}

/// This macro generates accessor functions for statically stored modules.
///
/// The macro takes a list of module names as input and generates corresponding accessor functions.
/// Each accessor function returns a reference to a statically stored `Module` instance.
///
/// # Parameters
///
/// - `$($name:ident),*`: A list of module names. Each name should be an identifier representing a module.
///
/// # Return
///
/// - A series of accessor functions, one for each module name provided.
///   Each function returns a reference to a statically stored `Module` instance.
///
/// # Safety
///
/// The macro assumes that the `MODULES` vector is properly initialized before calling the accessor functions.
/// If the `MODULES` vector is not initialized, calling an accessor function will result in a panic.
macro_rules! define_module_accessors {
    ($($name:ident),*) => {
        $(
            pub fn $name() -> &'static Module {
                unsafe {
                    MODULES.as_ref().expect(concat!(stringify!($name), " is not initialized"))
                        .iter()
                        .find(|module| module.name() == concat!(stringify!($name), ".dll"))
                        .expect(concat!(stringify!($name), " module not found"))
                }
            }
        )*
    };
}

define_module_accessors!(client, engine2, gameoverlayrenderer64);
