use crate::utils::find_window;
use anyhow::Context;

pub mod dx11;
pub mod fonts;
pub mod win32;

pub fn setup() -> anyhow::Result<()> {
    let window = find_window().context("could not find window")?;

    fonts::setup().context("failed to setup fonts")?;
    win32::setup(window).context("failed to setup WNDPROC hook")?;

    Ok(())
}
