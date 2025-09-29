use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    directories: Vec<String>,
}

pub fn read_config() -> Result<(), Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("todo");

    let config_path = xdg_dirs.place_config_file("config.toml")?;

    if !config_path.is_file() {
        println!("Creating config at {}", config_path.to_str().unwrap());
        File::create(config_path)?;
    }

    Ok(())
}
