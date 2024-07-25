use egui::Color32;
use lazy_static::lazy_static;
use parking_lot::Mutex;

lazy_static! {
    pub static ref SETTINGS: Mutex<Settings> = Mutex::new(Settings::default());
}

#[derive(PartialEq, Eq)]
pub enum Tab {
    Visuals,
    Misc,
}

pub struct Settings {
    pub tab: Tab,

    pub visuals: VisualsSettings,
    pub misc: MiscSettings,
}

impl Default for Settings {
    #[inline]
    fn default() -> Self {
        Self { tab: Tab::Visuals, visuals: Default::default(), misc: Default::default() }
    }
}

#[derive(Default)]
pub struct VisualsSettings {
    pub esp: EspSettings,
}

pub struct EspSettings {
    pub enabled: bool,
    pub draw_boxes: bool,
    pub box_color: Color32,
    pub draw_nametags: bool,
    pub draw_money: bool,
    pub draw_health: bool,
}

impl Default for EspSettings {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: true,
            draw_boxes: true,
            box_color: Color32::from_rgb(237, 135, 150),
            draw_nametags: true,
            draw_money: true,
            draw_health: true,
        }
    }
}

#[derive(Default)]
pub struct MiscSettings {}
