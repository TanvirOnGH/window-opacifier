extern crate anyhow;
use crate::config::Config;
use crate::utils;

use anyhow::Result;
use std::{process::Command, str::FromStr};

pub fn get_current_window_opacity() -> Result<u8> {
    let output = Command::new("picom-trans").args(["-c", "-g"]).output()?;

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

pub fn set_window_opacity(opacity: u8) {
    Command::new("picom-trans")
        .arg("-c")
        .arg("-o")
        .arg(&opacity.to_string())
        .spawn()
        .expect("Failed to start picom-trans");
}

pub fn animate_opacity(config: &Config, current_opacity: u8) {
    for opacity in (config.initial_opacity..=current_opacity).step_by(config.step_size as usize) {
        set_window_opacity(opacity);
        std::thread::sleep(std::time::Duration::from_millis(config.sleep_duration_ms));
    }
}

pub fn check_picom_process() -> Result<()> {
    let process_name = "picom";

    if !utils::is_process_running(process_name)? {
        eprintln!("Error: {} is not running.", process_name);
        std::process::exit(1);
    }

    Ok(())
}
