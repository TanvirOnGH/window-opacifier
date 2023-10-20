use anyhow::{Context, Result};
use std::{fs, io::Read, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub initial_opacity: u8,
    pub step_size: u8,
    pub sleep_duration_ms: u64,
}

pub fn read_config(config_path: &str) -> Result<Config> {
    let mut config_file = fs::File::open(config_path)
        .with_context(|| format!("Failed to open config file: {}", config_path))?;

    let mut config_content = String::new();
    config_file
        .read_to_string(&mut config_content)
        .with_context(|| format!("Failed to read config file: {}", config_path))?;

    let config: Config = toml::from_str(&config_content)
        .with_context(|| format!("Failed to parse TOML in config file: {}", config_path))?;

    Ok(config)
}

pub fn create_default_config(config_path: &str) -> Result<()> {
    let config = Config {
        initial_opacity: 1,
        step_size: 2,
        sleep_duration_ms: 12,
    };

    let toml_str =
        toml::to_string(&config).with_context(|| "Failed to serialize default config to TOML")?;

    if let Some(parent_dir) = Path::new(config_path).parent() {
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create config directory: {:?}", parent_dir))?;
    }

    fs::write(config_path, toml_str)
        .with_context(|| format!("Failed to write config to file: {}", config_path))?;

    Ok(())
}
