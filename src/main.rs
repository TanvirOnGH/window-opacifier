extern crate ctrlc;
use std::{
    io::{BufRead, BufReader, Error, ErrorKind, Result},
    process::{exit, Command, Stdio},
    str::FromStr,
    thread::sleep,
    time::Duration,
};

// TODO: Refactor code into modular structure
// TODO: Add documentation

fn get_current_window_opacity() -> Result<u8> {
    // Get the opacity of the current window using the 'picom-trans' command
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

    Err(Error::new(
        ErrorKind::Other,
        "Failed to get and parse integer from command output",
    ))
}

fn set_window_opacity(opacity: u8) {
    Command::new("picom-trans")
        .arg("-c")
        .arg("-o")
        .arg(&opacity.to_string())
        .spawn()
        .expect("Failed to start picom-trans");
}

fn is_process_running(process_name: &str) -> Result<bool> {
    let output = Command::new("pgrep")
        .arg(process_name)
        .stdout(Stdio::piped())
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

fn main() -> Result<()> {
    let process_name = "picom";

    if let Err(err) = is_process_running(process_name) {
        eprintln!("Error checking if picom is running: {}", err);
        exit(1);
    }

    // Define the initial opacity (TODO: read from config file)
    let initial_opacity: u8 = 1;

    let current_opacity = match get_current_window_opacity() {
        Ok(parsed_opacity) => parsed_opacity,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    };

    ctrlc::set_handler(move || {
        // Restore the original opacity and exit when a signal is received
        set_window_opacity(current_opacity);
        exit(0);
    })
    .expect("Failed to set signal handler");

    // Define the step size (TODO: read from config file)
    for opacity in (initial_opacity..=current_opacity).step_by(2) {
        set_window_opacity(opacity);
        // Define the sleep duration (TODO: read from config file)
        sleep(Duration::from_millis(12));
    }

    Ok(())
}
