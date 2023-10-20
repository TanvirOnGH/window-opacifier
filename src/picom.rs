//! Module for interacting with the picom compositor in the Window Opacifier application.
//!
//! This module provides functions for checking the picom process, getting and setting the window opacity,
//! and animating the opacity of the current window.
extern crate anyhow;
use crate::config::Config;
use crate::utils;

use anyhow::{Context, Result};
use std::{process::Command, str::FromStr};

/// Retrieves the current opacity of the window.
///
/// This function executes the 'picom-trans' command to obtain the current opacity of the active window.
///
/// # Errors
///
/// This function returns an error if it fails to execute the 'picom-trans' command or parse the output.
pub fn get_current_window_opacity() -> Result<u8> {
    let output = Command::new("picom-trans")
        .args(["-c", "-g"])
        .output()
        .with_context(|| "Failed to execute picom-trans command")?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Ok(parsed_opacity) = u8::from_str(output_str.trim()) {
            return Ok(parsed_opacity);
        } else {
            eprintln!("Failed to parse the output as an integer");
        }
    } else {
        eprintln!("Command failed with an error: {:?}", output.status);
    }

    Err(anyhow::anyhow!(
        "Failed to get and parse integer from command output",
    ))
}

/// Sets the opacity of the window.
///
/// This function uses the 'picom-trans' command to set the opacity of the active window to the specified value.
pub fn set_window_opacity(opacity: u8) {
    Command::new("picom-trans")
        .arg("-c")
        .arg("-o")
        .arg(&opacity.to_string())
        .spawn()
        .expect("Failed to start picom-trans");
}

/// Animates the opacity of the current window.
///
/// This function animates the opacity of the current window by gradually changing it according to
/// the specified configuration.
///
/// # Errors
///
/// This function returns an error if any of the underlying functions fail.
pub fn animate_opacity(config: &Config, current_opacity: u8) {
    for opacity in (config.initial_opacity..=current_opacity).step_by(config.step_size as usize) {
        set_window_opacity(opacity);
        std::thread::sleep(std::time::Duration::from_millis(config.sleep_duration_ms));
    }
}

/// Checks if the picom process is running.
///
/// This function verifies if the picom compositor process is running.
///
/// # Errors
///
/// This function returns an error if it fails to check the picom process status.
pub fn check_picom_process() -> Result<()> {
    let process_name = "picom";

    if !utils::is_process_running(process_name)
        .with_context(|| format!("Failed to check if {} process is running", process_name))?
    {
        eprintln!("Error: {} is not running.", process_name);
        std::process::exit(1);
    }

    Ok(())
}
