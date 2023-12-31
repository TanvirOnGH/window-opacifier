//! The main entry point of the Window Opacifier application.
extern crate ctrlc;
extern crate toml;

#[macro_use]
extern crate serde_derive;
extern crate anyhow;

mod config;
mod picom;
mod utils;

use anyhow::{Context, Result};

/// The main function that runs the Window Opacifier application.
///
/// # Errors
///
/// This function returns an error if any of the tasks fail.
fn main() -> Result<()> {
    let home_dir = dirs::home_dir()
        .ok_or(anyhow::anyhow!("Unable to determine home directory"))
        .with_context(|| "Failed to determine home directory")?;

    let config_path = home_dir.join(".config/window-opacifier/config.toml");

    if !config_path.exists() {
        eprintln!(
            "Configuration file not found at {}. Creating one with default values.",
            config_path.display()
        );
        config::create_default_config(config_path.to_str().unwrap())
            .with_context(|| "Failed to create default config")?;
    }

    let config = config::read_config(config_path.to_str().unwrap())
        .with_context(|| "Failed to read config file")?;

    picom::check_picom_process().with_context(|| "Failed to check picom process")?;

    let current_opacity = picom::get_current_window_opacity()
        .with_context(|| "Failed to get current window opacity")?;

    utils::set_signal_handler(current_opacity).with_context(|| "Failed to set signal handler")?;

    picom::animate_opacity(&config, current_opacity);

    Ok(())
}
