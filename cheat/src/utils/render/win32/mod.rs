use crate::common;
use common::*;

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

/// Initializes the input handling and menu system for the given window.
///
/// # Parameters
///
/// * `window`: A handle to the window for which input handling and menu system will be initialized.
///
/// # Returns
///
/// * `Result<(), anyhow::Error>`: Returns `Ok(())` if the initialization is successful.
///   Returns an error if the `WNDPROC` or `INPUT` is already initialized.
pub fn setup(window: HWND) -> anyhow::Result<()> {
    #[allow(clippy::fn_to_numeric_cast)]
    if WNDPROC
        .set(unsafe { transmute(SetWindowLongPtrA(window, GWLP_WNDPROC, wndproc_hk as isize)) })
        .is_err()
    {
        bail!("WNDPROC is already initialized");
    }

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
    INPUT.get().expect(&"INPUT is not initialized").lock().process(msg, wparam.0, lparam.0);

    let wndproc = WNDPROC.get().expect(&"WNDPROC is not initialized");

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
