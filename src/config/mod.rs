use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Directory {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    task_folder: Option<Vec<Directory>>,
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

pub fn save_config(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // todo config checking
    //

    let toml = toml::to_string(&config)?;
    Ok(())
}
