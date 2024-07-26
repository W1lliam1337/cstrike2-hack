use crate::{common, utils::module_handler};
use anyhow::bail;
use common::{c_void, Mutex};

use once_cell::sync::OnceCell;
use windows::Win32::Foundation::HMODULE;

/// A `Module` represents a dynamically loaded module.
///
/// This struct provides methods to interact with the module, such as finding sequences of bytes,
/// retrieving exported functions, and obtaining interfaces.
///
/// # Fields
/// - `name`: The name of the module.
/// - `handle`: The handle to the loaded module.
#[derive(Clone, Debug)]
pub struct Module {
    /// The name of the module.
    name: &'static str,

    /// The handle to the loaded module.
    handle: HMODULE,
}

impl Module {
    /// Creates a new `Module` by loading the module with the given name.
    ///
    /// # Parameters
    /// - `name`: The name of the module to load.
    ///
    /// # Returns
    /// A new `Module` instance.
    ///
    /// # Panics
    /// This function will panic if the module cannot be loaded.
    /// The panic occurs if `module_handler::get_module_handle(name)` returns `None`.
    ///
    /// # Examples
    /// ```
    /// let module = Module::new("example.dll");
    /// ```
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        let handle = module_handler::get_module_handle(name).expect("failed to get module handle");
        Self { name, handle }
    }

    /// Searches for a sequence of bytes in the module.
    ///
    /// # Parameters
    /// - `pattern`: The byte pattern to search for.
    ///
    /// # Returns
    /// The offset of the pattern if found, otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// let offset = module.find_seq_of_bytes("pattern").unwrap_or(0);
    /// ```
    #[must_use]
    pub fn find_seq_of_bytes<T>(&self, pattern: &str) -> anyhow::Result<*const T> {
        module_handler::pattern_search(self.handle, pattern)
    }

    /// Retrieves the address of an exported function from the module.
    ///
    /// # Parameters
    /// - `function_name`: The name of the function to retrieve.
    ///
    /// # Returns
    /// A pointer to the function if found, otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// let func_ptr = module.get_export("function_name");
    /// ```
    #[must_use]
    pub fn get_export(&self, function_name: &str) -> Option<*mut c_void> {
        module_handler::get_proc_address(self.handle, function_name)
    }

    /// Retrieves an interface from the module.
    ///
    /// # Parameters
    /// - `interface_name`: The name of the interface to retrieve.
    ///
    /// # Returns
    /// A pointer to the interface if found, otherwise `None`.
    ///
    /// # Examples
    /// ```
    /// let interface_ptr = module.get_interface("interface_name");
    /// ```
    #[must_use]
    pub fn get_interface(&self, interface_name: &str) -> Option<*const usize> {
        module_handler::get_interface(self.handle, interface_name)
    }

    /// Returns the name of the module.
    ///
    /// # Returns
    /// The name of the module.
    ///
    /// # Examples
    /// ```
    /// let module_name = module.name();
    /// ```
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }
}

/// A global static variable holding the list of initialized modules.
///
/// This variable is initialized only once and protected by a `Mutex` to ensure thread safety.
static MODULES: OnceCell<Mutex<Vec<Module>>> = OnceCell::new();

/// Initializes the global `MODULES` with the provided module names.
///
/// # Parameters
/// - `names`: A slice of module names to initialize.
///
/// # Returns
/// A `Result` indicating success or failure. If the initialization fails, it returns an error.
///
/// # Errors
/// - Returns an error if modules are already initialized.
/// - Panics if setting the global `MODULES` fails.
///
/// # Panics
/// This function will panic if `MODULES.set(...)` fails or if `MODULES.get()` returns `None`
/// while trying to access the modules. This can happen if the modules were not properly initialized.
///
/// # Examples
/// ```no_run
/// let result = initialize_modules(&["module1.dll", "module2.dll"]);
/// match result {
///     Ok(_) => println!("Modules initialized successfully"),
///     Err(e) => eprintln!("Failed to initialize modules: {:?}", e),
/// }
/// ```
pub fn initialize_modules(names: &[&'static str]) -> anyhow::Result<()> {
    if MODULES.get().is_some() {
        bail!("modules are already initialized");
    }

    let modules = names
        .iter()
        .map(|&name| {
            let module = Module::new(name);

            tracing::info!(
                "initialized module: {} {:p}",
                module.name,
                module.handle.0 as *const c_void
            );

            module
        })
        .collect();

    match MODULES.set(Mutex::new(modules)) {
        Ok(_) => {}
        Err(e) => bail!("failed to initialize MODULES: {e:?}"),
    }

    Ok(())
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
macro_rules! define_module_accessors {
    ($($name:ident),*) => {
        $(
            /// Accessor function for the module.
            ///
            /// # Panics
            /// Panics if the module is not initialized or if the module is not found.
            pub fn $name() -> &'static Module {
                let module_name = concat!(stringify!($name), ".dll");
                let modules_guard = MODULES.get().expect("modules are not initialized").lock();
                let module = modules_guard.iter()
                    .find(|module| module.name() == module_name)
                    .unwrap_or_else(|| {
                        panic!("module {} is not found", module_name);
                    });

                Box::leak(Box::new(module.clone()))
            }
        )*
    };
}

define_module_accessors!(client, engine2, gameoverlayrenderer64);
