use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

// todo, i probably think it'll be better to make this go into directories.toml instead
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub task_folder: Option<Vec<crate::handler::data::Directory>>,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("todo");

    let config_path = xdg_dirs.place_config_file("config.toml")?;

    if !config_path.is_file() {
        println!("Creating config at {}", config_path.to_str().unwrap());
        File::create(&config_path)?;
    }

    let config = std::fs::read_to_string(&config_path)?;

    let data: Config = toml::from_str(&config)?;

    Ok(data)
}

pub fn save_config(config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let converted_toml = toml::to_string(&config)?;
    let config_toml = converted_toml.into_bytes();

    let xdg_dirs = xdg::BaseDirectories::with_prefix("todo");

    let config_path = xdg_dirs.place_config_file("config.toml")?;

    let mut config_file = File::create(&config_path)?;

    config_file.write_all(&config_toml)?;
    Ok(())
}
