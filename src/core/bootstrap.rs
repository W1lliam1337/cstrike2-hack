use crate::{
    core::hooks,
    cs2::{self},
};

pub fn initialize() {
    println!("Initializing core components...");

    cs2::modules::initialize_modules(&["client.dll", "engine2.dll", "gameoverlayrenderer64.dll"]);
    hooks::initialize_hooks();
}
