use crate::core::ui;

use egui_d3d11::DirectX11App;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Dxgi::IDXGISwapChain,
    Graphics::Dxgi::DXGI_SWAP_CHAIN_DESC,
    UI::WindowsAndMessaging::{SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
};

pub static mut DX11: DirectX11App<i32> = DirectX11App::new();
pub static mut OLD_WND_PROC: Option<WNDPROC> = None;

/// Initializes the window procedure for the given swap chain description and a custom window procedure.
///
/// # Parameters
///
/// * `desc`: A reference to a `DXGI_SWAP_CHAIN_DESC` struct that describes the swap chain.
/// * `wnd_proc`: An unsafe extern "system" function pointer representing the custom window procedure.
///
/// # Safety
///
/// This function is unsafe because it interacts with the Windows API, which requires careful handling.
///
/// # Panics
///
/// If the `OutputWindow` field of the `DXGI_SWAP_CHAIN_DESC` struct is invalid (i.e., its value is -1),
/// this function will panic with the message "Invalid window handle".
///
/// # Remarks
///
/// This function sets the custom window procedure for the window associated with the swap chain.
/// It also stores the old window procedure in the `OLD_WND_PROC` static variable.
pub unsafe fn init_wnd_proc(
    desc: &DXGI_SWAP_CHAIN_DESC,
    wnd_proc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT,
) {
    if desc.OutputWindow.0 == -1 {
        panic!("Invalid window handle");
    }

    OLD_WND_PROC = Some(std::mem::transmute(SetWindowLongPtrA(
        desc.OutputWindow,
        GWLP_WNDPROC,
        wnd_proc as isize,
    )));
}

/// Initializes the render data for the given DirectX 11 swap chain.
///
/// This function initializes the default render data for the provided swap chain using the
/// `init_default` method of the `DirectX11App` struct. It also sets the custom drawing function
/// `ui::draw_menu` as the callback function for rendering the user interface.
///
/// # Safety
///
/// This function is unsafe because it interacts with the DirectX 11 API, which requires careful handling.
///
/// # Parameters
///
/// * `swap_chain`: A reference to an `IDXGISwapChain` interface representing the swap chain for which
///                 the render data will be initialized.
///
/// # Return
///
/// This function does not return any value.
pub unsafe fn init_render_data(swap_chain: &IDXGISwapChain) {
    DX11.init_default(swap_chain, ui::draw_menu);
}
