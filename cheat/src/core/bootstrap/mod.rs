use anyhow::Context;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::{
    core::hooks,
    cs2::{self},
    utils::render,
};

fn init_tracing() -> anyhow::Result<()> {
    let subscriber =
        FmtSubscriber::builder().with_max_level(Level::TRACE).with_ansi(false).finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("failed to set global default tracing subscriber")?;

    Ok(())
}

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
    tracing::info!("initializing core components...");

    init_tracing().context("failed to initialize tracing")?;

    cs2::modules::initialize_modules(&["client.dll", "engine2.dll", "gameoverlayrenderer64.dll"])
        .context("failed to initialize modules")?;

    render::setup().context("failed to setup renderer")?;

    hooks::initialize_hooks().context("failed to initialize hooks")?;

    Ok(())
}
