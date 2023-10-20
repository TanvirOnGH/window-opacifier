//! Module for working with configuration data in the Window Opacifier application.
//!
//! This module provides functionality to read and create a configuration file for the Window Opacifier
//! application. It defines the structure of the configuration data and supports serialization and
//! deserialization to/from TOML format.
use anyhow::{Context, Result};
use std::{fs, io::Read, path::Path};

// Struct representing the configuration data.
///
/// This struct defines the configuration parameters used by the Window Opacifier application. It
/// specifies the initial opacity, step size, and sleep duration in milliseconds.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub initial_opacity: u8,
    pub step_size: u8,
    pub sleep_duration_ms: u64,
}

/// Reads the configuration from a file.
///
/// This function reads the configuration data from a specified file in TOML format.
///
/// # Errors
///
/// This function returns an error if it fails to open, read, or parse the configuration file.
pub fn read_config(config_path: &str) -> Result<Config> {
    let mut config_file = fs::File::open(config_path)
        .with_context(|| format!("Failed to open config file: {}", config_path))?;

    let mut config_content = String::new();
    config_file
        .read_to_string(&mut config_content)
        .with_context(|| format!("Failed to read config file: {}", config_path))?;

    let config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse TOML in config file: {}", config_path))?;

    Ok(config)
}

/// Creates a default configuration file.
///
/// This function creates a default configuration file with predefined values and saves it in the
/// specified path.
///
/// # Errors
///
/// This function returns an error if it fails to serialize or write the default configuration to the file.
pub fn create_default_config(config_path: &str) -> Result<()> {
    let config = Config {
        initial_opacity: 1,
        step_size: 2,
        sleep_duration_ms: 12,
    };

    let toml_str =
        toml::to_string(&config).with_context(|| "Failed to serialize default config to TOML")?;

    if let Some(parent_dir) = Path::new(config_path).parent() {
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", parent_dir))?;
    }

    fs::write(config_path, toml_str)
        .with_context(|| format!("Failed to write config to file: {}", config_path))?;

    Ok(())
}
