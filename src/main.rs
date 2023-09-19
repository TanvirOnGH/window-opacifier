extern crate ctrlc;
use std::{
    io::{BufRead, BufReader, Result},
    process::{exit, Command, Stdio},
    str::FromStr,
    thread::sleep,
    time::Duration,
};

// TODO: Refactor code into modular structure
// TODO: Add documentation

fn get_current_window_opacity() -> Result<u8> {
    // TODO: Add error handling for Command::new() and Command::args()
    let output = Command::new("picom-trans").args(&["-c", "-g"]).output()?;

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

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to get and parse integer from command output",
    ))
}

fn set_window_opacity(opacity: u8) -> () {
    // TODO: Add error handling for Command::new() and Command::arg()
    Command::new("picom-trans")
        .arg("-c")
        .arg("-o")
        .arg(&opacity.to_string())
        .spawn()
        .expect("Failed to start picom-trans");
}

fn is_process_running(process_name: &str) -> bool {
    if let Ok(output) = Command::new("pgrep")
        .arg(process_name)
        .stdout(Stdio::piped())
        .spawn()
    {
        let reader = BufReader::new(output.stdout.unwrap());
        for line in reader.lines() {
            if let Ok(_) = line {
                return true;
            }
        }
    }

    false
}

fn main() -> Result<()> {
    let process_name = "picom";

    if !is_process_running(process_name) {
        eprintln!("{} is not running.", process_name);
        exit(1);
    }

    // TODO: Parse initial_opacity from config file
    let initial_opacity: u8 = 1;

    let current_opacity = match get_current_window_opacity() {
        Ok(parsed_opacity) => parsed_opacity,
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    };

    // TODO: Parse step_size from config file
    for opacity in (initial_opacity..=current_opacity).step_by(2) {
        set_window_opacity(opacity);
        // TODO: Parse sleep_duration from config file
        sleep(Duration::from_millis(12));
    }

    let _signal_handler = ctrlc::set_handler(move || {
        set_window_opacity(current_opacity);
        exit(0);
    })
    .expect("Failed to set signal handler");

    Ok(())
}
