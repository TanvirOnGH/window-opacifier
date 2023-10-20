use std::{error::Error, process::Command, str::FromStr};

pub fn get_current_window_opacity() -> Result<u8, Box<dyn Error>> {
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

    Err("Failed to get and parse integer from command output")?
}

pub fn set_window_opacity(opacity: u8) {
    Command::new("picom-trans")
        .arg("-c")
        .arg("-o")
        .arg(&opacity.to_string())
        .spawn()
        .expect("Failed to start picom-trans");
}
