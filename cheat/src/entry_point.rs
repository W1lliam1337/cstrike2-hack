pub mod common;
pub mod core;
pub mod cs2;
pub mod utils;

use common::*;

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
    if let Err(e) = core::bootstrap::initialize() {
        eprint!("Init failed: {}", e);
        return 0;
    } else {
        println!("Initialized cheat successfully!");
        return 0;
    }
}

/// The `dll_main` function is the entry point for a dynamic-link library (DLL) and is called by the operating system
/// when the DLL is loaded or unloaded. It is responsible for initializing and cleaning up the DLL.
///
/// # Parameters
///
/// - `module`: A pointer to the module handle for the DLL.
/// - `reason_for_call`: An unsigned integer representing the reason for calling the function.
///   The possible values are:
///   - `1`: DLL_PROCESS_ATTACH: The DLL is being loaded into a process.
///   - `0`: DLL_PROCESS_DETACH: The DLL is being unloaded from a process.
/// - `_reserved`: A pointer to reserved data. This parameter is not used and should be ignored.
///
/// # Return Value
///
/// The function should return a boolean value indicating success. If the function returns `1`, the DLL load is
/// successful. If the function returns `0`, the DLL load is unsuccessful.
#[export_name = "DllMain"]
pub extern "system" fn dll_main(
    _module: HMODULE,
    reason_for_call: u32,
    _reserved: *mut c_void,
) -> i32 {
    match reason_for_call {
        1 => {
            static INIT: Once = Once::new();

            INIT.call_once(|| {
                // Create a thread to initialize the cheat
                unsafe {
                    if AllocConsole().is_err() {
                        return;
                    }

                    CreateThread(
                        None,                     // Security attributes
                        0,                        // Stack size
                        Some(thread_startup),     // Thread function
                        Some(null_mut()),         // Parameter to thread function
                        THREAD_CREATION_FLAGS(0), // Creation flags
                        None,                     // Thread identifier
                    )
                    .unwrap();
                }
            });
        }
        0 => {
            println!("DLL unloaded");

            // TODO: Unload cheat and free resources
        }
        _ => {}
    }
    1 // TRUE
}
