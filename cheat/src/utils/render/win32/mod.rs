use crate::common;
use common::{transmute, Mutex, OnceLock};

use crate::{core::ui, utils::find_window};
use anyhow::{bail, Context};

use egui_win32::InputManager;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WM_KEYDOWN, WNDPROC,
    },
};

static WNDPROC: OnceLock<WNDPROC> = OnceLock::new();
pub static INPUT: OnceLock<Mutex<InputManager>> = OnceLock::new();

/// Sets up window procedure hooking and initializes the input manager.
///
/// # Parameters
///
/// - `window`: The handle to the window for which the procedure is set up.
///
/// # Returns
///
/// Returns an `anyhow::Result` which is `Ok(())` on success or an `Err` if any of the operations fail.
///
/// # Errors
///
/// - Returns an error if the window procedure (`WNDPROC`) is already initialized.
/// - Returns an error if the input manager (`INPUT`) is already initialized.
///
/// # Panics
///
/// This function does not panic. However, if the `SetWindowLongPtrA` function fails, it may cause undefined behavior.
pub fn setup(window: HWND) -> anyhow::Result<()> {
    // SAFETY:
    // - `wndproc_hk` is a valid function pointer with the correct signature.
    // - `SetWindowLongPtrA` expects a pointer to a window procedure, which is provided as `wndproc_hk` cast to `isize`.
    // - The returned `old_proc_ptr` from `SetWindowLongPtrA` is a valid pointer or `0` if the function fails.
    #[allow(clippy::fn_to_numeric_cast)]
    let old_proc_ptr = unsafe { SetWindowLongPtrA(window, GWLP_WNDPROC, wndproc_hk as isize) };

    // SAFETY: The cast to `isize` and back to a function pointer is managed by the API and is safe here.
    // We use `old_proc_ptr` to verify that the window procedure was successfully set.
    let wndproc_fn = unsafe {
        transmute::<isize, Option<unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>>(
            old_proc_ptr,
        )
    };

    if WNDPROC.set(wndproc_fn).is_err() {
        bail!("WNDPROC is already initialized");
    }

    // Initialize the input manager
    if INPUT.set(Mutex::new(InputManager::new(window))).is_err() {
        bail!("INPUT is already initialized");
    }

    Ok(())
}

/// Destroys the input handling and menu system for the application.
///
/// This function retrieves the window handle, checks if the `WNDPROC` and `INPUT` are initialized,
/// and then restores the original `WNDPROC` to the window.
///
/// # Returns
///
/// * `Result<(), anyhow::Error>`: Returns `Ok(())` if the destruction is successful.
///   Returns an error if the `WNDPROC` or `INPUT` is not initialized.
pub fn destroy() -> anyhow::Result<()> {
    let window = find_window().context("could not find window")?;

    let Some(Some(wndproc)) = WNDPROC.get() else {
        bail!("WNDPROC is not initialized");
    };

    // SAFETY: The `SetWindowLongPtrA` function is used here to set the window procedure, which requires a valid function pointer.
    #[allow(clippy::fn_to_numeric_cast)]
    unsafe {
        SetWindowLongPtrA(window, GWLP_WNDPROC, *wndproc as isize);
    };

    Ok(())
}

unsafe extern "system" fn wndproc_hk(
    window: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    INPUT.get().expect("INPUT is not initialized").lock().process(msg, wparam.0, lparam.0);

    let wndproc = WNDPROC.get().expect("WNDPROC is not initialized");

    match msg {
        WM_KEYDOWN if wparam.0 == 0x2D => {
            ui::toggle_menu(); // Toggle menu visibility
        }
        _ => (),
    }

    // Check if the menu is open and block input if necessary
    if ui::should_block_input(msg) {
        return LRESULT(1);
    }

    CallWindowProcW(*wndproc, window, msg, wparam, lparam)
}
