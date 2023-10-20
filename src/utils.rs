use crate::picom;
use std::{error::Error, io::BufRead, io::BufReader, process::Command};

pub fn is_process_running(process_name: &str) -> Result<bool, Box<dyn Error>> {
    let output = Command::new("pgrep")
        .arg(process_name)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

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

pub fn set_signal_handler(current_opacity: u8) -> Result<(), Box<dyn std::error::Error>> {
    // Restore the original opacity and exit when a signal is received
    ctrlc::set_handler(move || {
        picom::set_window_opacity(current_opacity);
        std::process::exit(0);
    })?;

    Ok(())
}
