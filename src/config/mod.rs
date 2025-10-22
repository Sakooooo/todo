use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

// todo, i probably think it'll be better to make this go into directories.toml instead
#[derive(Debug, Deserialize, Serialize)]
pub struct DirectoryConfig {
    pub task_folder: Option<Vec<Directory>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
}

pub fn read_directory_config() -> Result<DirectoryConfig, Box<dyn std::error::Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("todo");

    let directory_config_path = xdg_dirs.place_config_file("directories.toml")?;

    if !directory_config_path.is_file() {
        println!(
            "Creating directory config at {}",
            directory_config_path.to_str().unwrap()
        );
        File::create(&directory_config_path)?;
    }

    let directory_config = std::fs::read_to_string(&directory_config_path)?;

    let data: DirectoryConfig = toml::from_str(&directory_config)?;

    Ok(data)
}

pub fn save_directory_config(
    directory_config: &mut DirectoryConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let converted_toml = toml::to_string(&directory_config)?;
    let config_toml = converted_toml.into_bytes();

    let xdg_dirs = xdg::BaseDirectories::with_prefix("todo");

    let directory_config_path = xdg_dirs.place_config_file("directories.toml")?;

    let mut directory_config_file = File::create(&directory_config_path)?;

    directory_config_file.write_all(&config_toml)?;
    Ok(())
}
