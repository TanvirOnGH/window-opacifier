extern crate toml;

use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub initial_opacity: u8,
    pub step_size: u8,
    pub sleep_duration_ms: u64,
}

pub fn read_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_file = File::open(config_path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    let config: Config = toml::from_str(&config_content)?;

    Ok(config)
}
