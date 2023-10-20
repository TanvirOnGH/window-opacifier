extern crate ctrlc;
extern crate toml;

#[macro_use]
extern crate serde_derive;
extern crate anyhow;

mod config;
mod picom;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    let process_name = "picom";

    if !utils::is_process_running(process_name)? {
        eprintln!("Error: {} is not running.", process_name);
        std::process::exit(1);
    }

    let home_dir = dirs::home_dir().ok_or(anyhow::anyhow!("Unable to determine home directory"))?;
    let config_path = home_dir.join(".config/window-opacifier/config.toml");

    if !config_path.exists() {
        return Err(anyhow::anyhow!(
            "Configuration file not found at {}",
            config_path.display()
        ));
    }

    let config = config::read_config(config_path.to_str().unwrap())?;

    let current_opacity = picom::get_current_window_opacity()?;

    utils::set_signal_handler(current_opacity)?;
    picom::animate_opacity(&config, current_opacity);

    Ok(())
}
