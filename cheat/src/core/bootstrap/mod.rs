use crate::{
    core::hooks,
    cs2::{self},
    utils::render,
};

pub fn initialize() -> anyhow::Result<()> {
    println!("Initializing core components...");

    cs2::modules::initialize_modules(&["client.dll", "engine2.dll", "gameoverlayrenderer64.dll"]);
    render::setup()?;
    hooks::initialize_hooks()?;

    Ok(())
}
