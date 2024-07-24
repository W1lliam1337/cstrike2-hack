use anyhow::Context;
use egui::{FontData, FontDefinitions, FontFamily};
use parking_lot::Mutex;

pub static FONTS: Mutex<Option<FontDefinitions>> = Mutex::new(None);

pub fn setup() -> anyhow::Result<()> {
    let mut fonts = FontDefinitions::default();

    fonts
        .font_data
        .insert("Tahoma".to_owned(), FontData::from_static(include_bytes!("./tahoma.ttf")));

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .context("could not setup proportional fonts")?
        .insert(0, "Tahoma".to_owned());

    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .context("could not setup monospace fonts")?
        .insert(0, "Tahoma".to_owned());

    *FONTS.lock() = Some(fonts);

    Ok(())
}
