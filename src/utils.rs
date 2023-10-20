//! Module for utility functions in the Window Opacifier application.
//!
//! This module provides various utility functions used in the Window Opacifier application, including
//! checking if a process is running and setting a signal handler for graceful termination.
use crate::picom;

use anyhow::{Context, Result};
use std::{
    io::{BufRead, BufReader},
    process::Command,
};

/// Checks if a process is running.
///
/// This function checks if a specified process is currently running.
///
/// # Errors
///
/// This function returns an error if it fails to execute the 'pgrep' command or read the process list.
pub fn is_process_running(process_name: &str) -> Result<bool> {
    let output = Command::new("pgrep")
        .arg(process_name)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to execute pgrep for process: {}", process_name))?;

    if let Some(output) = output.stdout {
        let reader = BufReader::new(output);
        for line in reader.lines() {
            if line.is_ok() {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

/// Sets a signal handler for graceful termination.
///
/// This function sets a signal handler to gracefully exit the application when a termination signal
/// is received, restoring the original window opacity.
///
/// # Errors
///
/// This function returns an error if it fails to set the signal handler.
pub fn set_signal_handler(current_opacity: u8) -> Result<()> {
    ctrlc::set_handler(move || {
        // Restore the original opacity
        picom::set_window_opacity(current_opacity);
        std::process::exit(0);
    })
    .with_context(|| "Failed to set signal handler")?;

    Ok(())
}
