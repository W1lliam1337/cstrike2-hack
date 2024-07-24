pub mod hook_system;
pub mod module_handler;
pub mod render;

use windows::Win32::{
    Foundation::{BOOL, FALSE, HWND, LPARAM, TRUE},
    System::{Console::GetConsoleWindow, Threading::GetCurrentProcessId},
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindow, GetWindowThreadProcessId, IsWindowVisible, GW_OWNER,
    },
};

/// Determines whether a given window is the main window of the current process.
///
/// This function checks if the specified window is a top-level window, has no owner, and is visible.
/// It is intended to be used as a helper function for identifying the main window of the current process.
///
/// # Parameters
///
/// * `window`: A handle to the window to be checked.
///
/// # Returns
///
/// * `true` if the specified window is the main window of the current process.
/// * `false` if the specified window is not the main window of the current process.
unsafe fn is_main_window(window: HWND) -> bool {
    GetWindow(window, GW_OWNER).0 == 0 && IsWindowVisible(window).into()
}

/// An unsafe extern "system" function used as a callback for the `EnumWindows` function.
/// This function is intended to identify the main window of the current process.
///
/// # Parameters
///
/// * `window`: A handle to the window being enumerated.
/// * `lparam`: A pointer to a mutable memory location where the function can store the handle to the main window.
///
/// # Returns
///
/// * `BOOL`: A boolean value indicating whether the enumeration should continue.
///   - `TRUE`: The enumeration should continue.
///   - `FALSE`: The enumeration should stop. In this case, the function has found the main window and stored its handle in `lparam`.
unsafe extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
    let mut window_proc_id = 0;
    let _ = GetWindowThreadProcessId(window, Some(&mut window_proc_id));

    if GetCurrentProcessId() != window_proc_id
        || !is_main_window(window)
        || window == GetConsoleWindow()
    {
        return TRUE;
    }

    let lparam = std::mem::transmute::<_, *mut HWND>(lparam);

    *lparam = window;

    FALSE
}

/// Finds the main window of the current process.
///
/// This function uses the `EnumWindows` function to iterate through all top-level windows in the system.
/// It then checks each window to determine if it is the main window of the current process.
/// The main window is defined as a visible window without an owner and is not the console window.
///
/// # Returns
///
/// * `Some(HWND)` - If a main window is found, the function returns the handle to the main window.
/// * `None` - If no main window is found, the function returns `None`.
pub fn find_window() -> Option<HWND> {
    let mut hwnd: HWND = Default::default();

    let _ = unsafe { EnumWindows(Some(enum_window), LPARAM(&mut hwnd as *mut HWND as isize)) };

    if hwnd.0 == 0 {
        None
    } else {
        Some(hwnd)
    }
}
