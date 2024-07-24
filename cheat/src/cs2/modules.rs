use crate::{common, utils::module_handler};
use common::*;

use once_cell::sync::OnceCell;
use windows::Win32::Foundation::HMODULE;

#[derive(Clone, Debug)]
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

static MODULES: OnceCell<Mutex<Vec<Module>>> = OnceCell::new();

/// Initializes the static instances of the `Module` struct with the given module names.
/// This function ensures that the modules are only initialized once, and provides access to the initialized modules through accessor functions.
///
/// # Arguments
///
/// * `names` - A slice of module names (without the ".dll" extension) for which the `Module` instances will be initialized.
pub fn initialize_modules(names: &[&'static str]) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let modules = names.iter().map(|&name| Module::new(name)).collect::<Vec<_>>();
        MODULES.set(Mutex::new(modules)).expect("Failed to initialize MODULES");

        let modules = MODULES.get().expect("MODULES should be initialized").lock();
        for module in modules.iter() {
            println!("Initialized module: {} {:p}", module.name, module.handle.0 as *const c_void);
        }
    });
}

/// This macro generates accessor functions for static instances of the `Module` struct.
/// These functions allow easy access to the initialized modules without needing to manually manage their lifetimes.
///
/// # Arguments
///
/// * `$($name:ident),*` - A list of module names (without the ".dll" extension) for which accessor functions will be generated.
///
/// # Example
///
/// ```rust
/// define_module_accessors!(client, engine2, gameoverlayrenderer64);
///
/// fn main() {
///     let client_module = client();
///     let engine2_module = engine2();
///     let gameoverlayrenderer64_module = gameoverlayrenderer64();
/// }
/// ```
macro_rules! define_module_accessors {
    ($($name:ident),*) => {
        $(
            pub fn $name() -> &'static Module {
                let modules = MODULES.get().expect(concat!(stringify!($name), " is not initialized")).lock();
                let module = modules.iter()
                    .find(|module| module.name() == concat!(stringify!($name), ".dll"))
                    .unwrap_or_else(|| {
                        panic!("Module {} is not found", stringify!($name));
                    });

                Box::leak(Box::new(module.clone()))
            }
        )*
    };
}

define_module_accessors!(client, engine2, gameoverlayrenderer64);
