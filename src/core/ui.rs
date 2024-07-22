#[allow(unused_imports)]
use egui::{Color32, Context, Pos2, Rect, RichText, ScrollArea, Slider, Stroke, Widget};

use windows::Win32::UI::WindowsAndMessaging::{
    WM_CHAR, WM_DEVICECHANGE, WM_KEYDOWN, WM_KEYUP, WM_KILLFOCUS, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN,
    WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE,
    WM_MOUSEWHEEL, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP,
    WM_SETCURSOR, WM_SETFOCUS, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN,
    WM_XBUTTONUP,
};

static mut SHOW_MENU: bool = true;

/// Toggles the visibility of the menu.
///
/// This function toggles the value of the `SHOW_MENU` static variable, which controls the visibility of the menu.
/// When `SHOW_MENU` is `true`, the menu will be displayed; when `SHOW_MENU` is `false`, the menu will be hidden.
///
/// # Safety
///
/// This function is marked as `unsafe` because it directly manipulates a static mutable variable, which can lead to data races.
/// It is assumed that the caller has ensured that the static variable is accessed in a thread-safe manner.
pub unsafe fn toggle_menu() {
    SHOW_MENU = !SHOW_MENU;
}

/// Renders the menu UI using the provided Egui context and mutable integer reference.
///
/// This function checks the value of the `SHOW_MENU` static variable and returns early if it is `false`.
/// It then initializes static mutable variables for UI elements and renders the menu using Egui.
///
/// # Parameters
///
/// * `ctx`: A reference to the Egui context used for rendering the menu.
/// * `i`: A mutable reference to an integer value that is used within the menu rendering.
pub fn draw_menu(ctx: &Context, i: &mut i32) {
    if !unsafe { SHOW_MENU } {
        return;
    }

    // You should not use statics like this, it made
    // this way for the sake of example.
    static mut UI_CHECK: bool = true;
    static mut TEXT: Option<String> = None;
    static mut VALUE: f32 = 0.;
    static mut COLOR: [f32; 3] = [0., 0., 0.];

    unsafe {
        if TEXT.is_none() {
            TEXT = Some(String::from("Test"));
        }
    }

    egui::containers::Window::new("Enigma Euphoria").show(ctx, |ui| {
        ui.label(RichText::new("Contact developer: t.me/animstate").color(Color32::WHITE));
        ui.separator();

        ui.label(RichText::new(format!("I: {}", *i)).color(Color32::LIGHT_RED));

        unsafe {
            ui.checkbox(&mut UI_CHECK, "Some checkbox");
            ui.text_edit_singleline(TEXT.as_mut().unwrap());
            ScrollArea::vertical().max_height(200.).show(ui, |ui| {
                for i in 1..=100 {
                    ui.label(format!("Label: {}", i));
                }
            });

            Slider::new(&mut VALUE, -1.0..=1.0).ui(ui);

            ui.color_edit_button_rgb(&mut COLOR);
        }

        ui.label(format!("{:?}", &ui.input().pointer.button_down(egui::PointerButton::Primary)));
        if ui.button("You can't click me yet").clicked() {
            *i += 1;
        }
    });
}

/// Determines whether input events should be blocked for a specific window message.
///
/// This function checks if the given window message `msg` corresponds to any of the input events
/// that should be blocked. The function returns `true` if the input event should be blocked,
/// and `false` otherwise.
///
/// # Parameters
///
/// * `msg`: A `u32` representing the window message to be checked.
///
/// # Return
///
/// * `bool`: A boolean value indicating whether the input event corresponding to the given window message
///           should be blocked.
pub fn should_block_input(msg: u32) -> bool {
    unsafe {
        matches!(SHOW_MENU, true if {
            matches!(
                msg,
                WM_MOUSEMOVE
                    | WM_NCMOUSEMOVE
                    | WM_NCMOUSELEAVE
                    | WM_LBUTTONDOWN
                    | WM_LBUTTONDBLCLK
                    | WM_RBUTTONDOWN
                    | WM_RBUTTONDBLCLK
                    | WM_MBUTTONDOWN
                    | WM_MBUTTONDBLCLK
                    | WM_XBUTTONDOWN
                    | WM_XBUTTONDBLCLK
                    | WM_LBUTTONUP
                    | WM_RBUTTONUP
                    | WM_MBUTTONUP
                    | WM_XBUTTONUP
                    | WM_MOUSEWHEEL
                    | WM_MOUSEHWHEEL
                    | WM_KEYDOWN
                    | WM_KEYUP
                    | WM_SYSKEYDOWN
                    | WM_SYSKEYUP
                    | WM_SETFOCUS
                    | WM_KILLFOCUS
                    | WM_CHAR
                    | WM_SETCURSOR
                    | WM_DEVICECHANGE
            )
        })
    }
}
