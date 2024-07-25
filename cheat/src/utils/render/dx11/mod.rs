use crate::{
    common,
    core::{settings, ui},
};

use common::{Mutex, OnceLock};

use anyhow::Context;
use egui_directx11::DirectX11Renderer;

use windows::Win32::{
    Foundation::{HMODULE, HWND, TRUE},
    Graphics::{
        Direct3D::{D3D_DRIVER_TYPE_NULL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_11_1},
        Direct3D11::{
            D3D11CreateDeviceAndSwapChain, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION,
        },
        Dxgi::{
            Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_SAMPLE_DESC},
            IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_USAGE_RENDER_TARGET_OUTPUT,
        },
    },
};

use super::{fonts, win32};

pub static DX11: OnceLock<Mutex<DirectX11Renderer>> = OnceLock::new();

/// Creates a DirectX 11 swap chain for the given window handle.
///
/// # Parameters
///
/// * `window`: A handle to the window where the swap chain will be displayed.
///
/// # Returns
///
/// * `Result<IDXGISwapChain>`: On success, returns the created swap chain.
///   On error, returns an `anyhow::Result` containing the error.
#[allow(dead_code)]
fn create_swapchain(window: HWND) -> anyhow::Result<IDXGISwapChain> {
    let flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
    let feature_levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_10_0];

    let swapchain_description = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC { Format: DXGI_FORMAT_R8G8B8A8_UNORM, ..Default::default() },
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, ..Default::default() },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        OutputWindow: window,
        Windowed: TRUE,
        ..Default::default()
    };

    let mut device = None;
    let mut swapchain: Option<IDXGISwapChain> = None;

    // SAFETY:
    // - The pointers passed to `D3D11CreateDeviceAndSwapChain` must be valid and correctly set up.
    // - All parameters should be properly initialized before calling this function.
    // - Proper error handling is in place to manage any issues from the FFI call.
    unsafe {
        D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_NULL,
            HMODULE::default(),
            flags,
            Some(&feature_levels),
            D3D11_SDK_VERSION,
            Some(&swapchain_description),
            Some(&mut swapchain),
            Some(&mut device),
            None,
            None,
        )
        .context("D3D11CreateDeviceAndSwapChain failed")?;
    };

    swapchain.context("could not create d3d11 swapchain")
}

/// Initializes the DirectX 11 renderer from the given swap chain.
///
/// This function sets up the DirectX 11 renderer, collects input from the `win32::INPUT` module,
/// and locks the `settings::SETTINGS` mutex. It then attempts to paint the UI using the provided
/// closure, which includes setting fonts, modifying tessellation options, and drawing the menu.
/// If an error occurs during rendering, it logs the error message.
///
/// # Parameters
///
/// * `swapchain`: A reference to the DirectX 11 swap chain used for rendering.
///
/// # Panics
///
/// This function will panic if:
/// - The DirectX 11 renderer could not be initialized (`expect("could not create dx11 renderer")`).
/// - The `win32::INPUT` is not initialized (`expect("win32::INPUT is not initialized")`).
/// - The input collection failed (`expect("could not collect input")`).
/// - An error occurs during the `renderer.paint` call (`eprintln!("Rendering error: {e}")`).
///
/// # Return
///
/// This function does not return a value.
#[inline]
pub fn init_from_swapchain(swapchain: &IDXGISwapChain) {
    let mut renderer = DX11
        .get_or_init(|| {
            Mutex::new(
                DirectX11Renderer::init_from_swapchain(&swapchain, egui::Context::default())
                    .expect("could not create dx11 renderer"),
            )
        })
        .lock();

    let input = win32::INPUT
        .get()
        .expect("win32::INPUT is not initialized")
        .lock()
        .collect_input()
        .expect("could not collect input");

    let mut settings = settings::SETTINGS.lock();

    if let Err(e) = renderer.paint(swapchain, &mut settings, input, |ctx, settings| {
        match fonts::FONTS.lock().as_ref() {
            Some(fonts) => {
                ctx.set_fonts(fonts.clone());
                ctx.tessellation_options_mut(|options| {
                    options.feathering = false;
                });
                ui::draw_menu(ctx, settings);
            }
            None => {
                eprintln!("Fonts are not set up");
            }
        }
    }) {
        eprintln!("Rendering error: {e}");
    }
}
