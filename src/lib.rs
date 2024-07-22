pub mod common;
pub mod core;
pub mod cs2;
pub mod utils;

use common::*;
use winapi::um::{consoleapi, libloaderapi, wincon};

/// This function is responsible for initializing the cheat.
/// It is called as a thread function when the DLL is loaded into a process.
///
/// # Parameters
///
/// None.
///
/// # Return Value
///
/// Returns a `DWORD` value of 0. This value is not used by the operating system.
extern "system" fn thread_startup(_: *mut c_void) -> DWORD {
    core::bootstrap::initialize();
    0
}

/// The `DllMain` function is the entry point for a dynamic-link library (DLL) and is called by the operating system
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
#[no_mangle]
pub extern "system" fn DllMain(
    module: *mut c_void,
    reason_for_call: u32,
    _reserved: *mut c_void,
) -> i32 {
    match reason_for_call {
        1 => {
            static INIT: Once = Once::new();

            INIT.call_once(|| {
                // Create a thread to initialize the cheat
                unsafe {
                    libloaderapi::DisableThreadLibraryCalls(module as *mut HINSTANCE__);
                    consoleapi::AllocConsole();

                    winapi::um::processthreadsapi::CreateThread(
                        null_mut(),           // Security attributes
                        0,                    // Stack size
                        Some(thread_startup), // Thread function
                        null_mut(),           // Parameter to thread function
                        0,                    // Creation flags
                        null_mut(),           // Thread identifier
                    );
                }
            });
        }
        0 => {
            println!("DLL unloaded");

            // TODO: Unload cheat and free resources
            unsafe { wincon::FreeConsole() };
        }
        _ => {}
    }
    1 // TRUE
}
