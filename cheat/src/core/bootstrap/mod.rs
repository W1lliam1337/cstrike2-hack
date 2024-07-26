use crate::{
    core::hooks,
    cs2::{self},
    utils::render,
};

/// Initializes the core components of the cheat.
///
/// This function sets up the necessary modules, rendering, and hooks for the cheat to function.
///
/// # Parameters
///
/// None.
///
/// # Returns
///
/// * `Result<(), anyhow::Error>`:
///   - `Ok(())`: Indicates that the initialization was successful.
///   - `Err(e)`: Returns an error if any of the initialization steps fail. The error type is `anyhow::Error`.
///
/// # Errors
///
/// This function may return the following errors:
///
/// * `anyhow::Error`: If any of the initialization steps (`initialize_modules`, `setup`, `initialize_hooks`) fail.
pub fn initialize() -> anyhow::Result<()> {
    println!("Initializing core components...");

    cs2::modules::initialize_modules(&["client.dll", "engine2.dll", "gameoverlayrenderer64.dll"])?;
    render::setup()?;
    hooks::initialize_hooks()?;

    Ok(())
}
