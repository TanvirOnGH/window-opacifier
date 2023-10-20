extern crate anyhow;
extern crate toml;

use anyhow::Result;
use std::{fs, io::Read, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub initial_opacity: u8,
    pub step_size: u8,
    pub sleep_duration_ms: u64,
}

pub fn read_config(config_path: &str) -> Result<Config> {
    let mut config_file = fs::File::open(config_path)?;
    let mut config_content = String::new();
    config_file.read_to_string(&mut config_content)?;
    let config: Config = toml::from_str(&config_content)?;

    Ok(config)
}

pub fn create_default_config(config_path: &str) -> Result<()> {
    let config = Config {
        initial_opacity: 1,
        step_size: 2,
        sleep_duration_ms: 12,
    };

    let toml_str = toml::to_string(&config)?;

    if let Some(parent_dir) = Path::new(config_path).parent() {
        fs::create_dir_all(parent_dir)?;
    }

    fs::write(config_path, toml_str)?;

    Ok(())
}
