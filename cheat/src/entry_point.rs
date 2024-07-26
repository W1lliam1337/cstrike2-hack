pub mod common;
pub mod core;
pub mod cs2;
pub mod utils;

use common::{c_void, null_mut, Once};

use windows::Win32::{
    Foundation::HMODULE,
    System::{
        Console::AllocConsole,
        Threading::{CreateThread, THREAD_CREATION_FLAGS},
    },
};

/// This function is responsible for initializing the cheat.
/// It is called as a thread function when the DLL is loaded into a process.
///
/// # Parameters
///
/// None.
///
/// # Return Value
///
/// Returns a `u32` value of 0. This value is not used by the operating system.
extern "system" fn thread_startup(_: *mut c_void) -> u32 {
    match core::bootstrap::initialize() {
        Err(e) => {
            tracing::error!("init failed: {e}");
        }
        Ok(()) => {
            tracing::info!("initialized cheat successfully!");
        }
    }

    0
}

/// The `dll_main` function is the entry point for a dynamic-link library (DLL) and is called by the operating system
/// when the DLL is loaded or unloaded. It is responsible for initializing and cleaning up the DLL.
///
/// # Parameters
///
/// - `module`: A pointer to the module handle for the DLL.
/// - `reason_for_call`: An unsigned integer representing the reason for calling the function.
///   The possible values are:
///   - `1`: `DLL_PROCESS_ATTACH`: The DLL is being loaded into a process.
///   - `0`: `DLL_PROCESS_DETACH`: The DLL is being unloaded from a process.
/// - `_reserved`: A pointer to reserved data. This parameter is not used and should be ignored.
///
/// # Return Value
///
/// The function should return a boolean value indicating success. If the function returns `1`, the DLL load is
/// successful. If the function returns `0`, the DLL load is unsuccessful.
///
/// # Panics
///
/// This function will panic if creating a thread fails.
#[export_name = "DllMain"]
pub extern "system" fn dll_main(
    _module: HMODULE,
    reason_for_call: u32,
    _reserved: *mut c_void,
) -> i32 {
    match reason_for_call {
        1 => {
            /// A static initializer to ensure one-time initialization.
            static INIT: Once = Once::new();

            INIT.call_once(|| {
                // Create a thread to initialize the cheat
                // SAFETY: AllocConsole is unsafe because it involves system-level operations that can fail.
                unsafe {
                    if AllocConsole().is_err() {
                        return;
                    }
                }

                // SAFETY: CreateThread is unsafe because it involves creating a new thread at the OS level.
                match unsafe {
                    CreateThread(
                        None,                     // Security attributes
                        0,                        // Stack size
                        Some(thread_startup),     // Thread function
                        Some(null_mut()),         // Parameter to thread function
                        THREAD_CREATION_FLAGS(0), // Creation flags
                        None,                     // Thread identifier
                    )
                } {
                    Ok(_) => {
                        tracing::info!("successfully created a new thread");
                    }
                    Err(e) => {
                        tracing::error!("failed to create thread: {:?}", e);
                    }
                }
            });
        }
        0 => {
            tracing::info!("DLL unloaded");

            // TODO: Unload cheat and free resources
        }
        _ => {}
    }
    1 // TRUE
}
