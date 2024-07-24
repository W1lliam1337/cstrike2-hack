use crate::core::settings::{Settings, Tab, VisualsSettings};

#[allow(unused_imports)]
use egui::{
    Color32, Context, Pos2, Rect, RichText, ScrollArea, Slider, Stroke, Ui, Widget, Window,
};

use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::UI::WindowsAndMessaging::{
    WM_CHAR, WM_DEVICECHANGE, WM_KEYDOWN, WM_KEYUP, WM_KILLFOCUS, WM_LBUTTONDBLCLK, WM_LBUTTONDOWN,
    WM_LBUTTONUP, WM_MBUTTONDBLCLK, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE,
    WM_MOUSEWHEEL, WM_NCMOUSELEAVE, WM_NCMOUSEMOVE, WM_RBUTTONDBLCLK, WM_RBUTTONDOWN, WM_RBUTTONUP,
    WM_SETCURSOR, WM_SETFOCUS, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_XBUTTONDBLCLK, WM_XBUTTONDOWN,
    WM_XBUTTONUP,
};

static SHOW_MENU: AtomicBool = AtomicBool::new(true);

/// Toggles the visibility of the menu.
///
/// This function toggles the visibility state of the menu by reading the current value of the
/// `SHOW_MENU` atomic boolean variable and negating it. The function then updates the value of
/// `SHOW_MENU` with the new visibility state.
///
/// # Parameters
///
/// None.
///
/// # Return
///
/// None.
pub fn toggle_menu() {
    let current = SHOW_MENU.load(Ordering::SeqCst);
    SHOW_MENU.store(!current, Ordering::SeqCst);
}

/// Checks if the menu is currently visible.
///
/// This function retrieves the current visibility state of the menu by reading the value of the
/// `SHOW_MENU` atomic boolean variable. The function returns `true` if the menu is visible, and
/// `false` otherwise.
///
/// # Parameters
///
/// None.
///
/// # Return
///
/// * `bool`: A boolean value indicating the visibility state of the menu.
pub fn is_menu_visible() -> bool {
    SHOW_MENU.load(Ordering::SeqCst)
}

/// Draws the main menu window with various settings options.
///
/// This function checks if the menu is currently visible using the `is_menu_visible` function. If the menu is
/// not visible, the function returns early without drawing anything. Otherwise, it creates a new window with
/// the title "enigma euphoria" and displays it using the provided `Context`. The window contains a label
/// with a contact link, a separator, and two tabs: "visuals" and "misc". Depending on the current tab
/// selected in the `Settings` struct, the corresponding tab function (`visuals_tab` or `misc_tab`) is
/// called to draw the specific settings options for that tab.
///
/// # Parameters
///
/// * `ctx`: A reference to the `Context` struct used for drawing UI elements.
/// * `settings`: A mutable reference to the `Settings` struct containing the current settings and tab
///               selection.
pub fn draw_menu(ctx: &Context, settings: &mut Settings) {
    if !is_menu_visible() {
        return;
    }

    Window::new("enigma euphoria").show(ctx, |ui| {
        ui.label(RichText::new("contact dev: t.me/animstate").color(Color32::WHITE));
        ui.separator();

        tabs(ui, settings);

        match settings.tab {
            Tab::Visuals => visuals_tab(ui, &mut settings.visuals),
            Tab::Misc => visuals_tab(ui, &mut settings.visuals),
        }
    });
}

fn tabs(ui: &mut Ui, settings: &mut Settings) {
    ui.horizontal(|ui| {
        if ui.selectable_label(settings.tab == Tab::Visuals, "visuals").clicked() {
            settings.tab = Tab::Visuals;
        }

        if ui.selectable_label(settings.tab == Tab::Misc, "misc").clicked() {
            settings.tab = Tab::Misc;
        }
    });
}

fn visuals_tab(ui: &mut Ui, settings: &mut VisualsSettings) {
    ui.label("esp");

    ui.checkbox(&mut settings.esp.enabled, "enable");

    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.esp.draw_boxes, "box");
        ui.color_edit_button_srgba(&mut settings.esp.box_color);
    });

    ui.checkbox(&mut settings.esp.draw_nametags, "name");
    ui.checkbox(&mut settings.esp.draw_health, "health");
    ui.checkbox(&mut settings.esp.draw_money, "money");
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
    let menu_visible = SHOW_MENU.load(Ordering::SeqCst);

    matches!(menu_visible, true if {
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
