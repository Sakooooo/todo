use crate::{config, handler::data};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod errors;

pub fn get_todos(
    directory: &config::Directory,
) -> Result<Vec<data::Task>, Box<dyn std::error::Error>> {
    let target = Path::new(&directory.path);
    let paths = fs::read_dir(target)?;

    let mut valid_directories: Vec<PathBuf> = vec![];

    for path in paths.into_iter() {
        if path.as_ref().unwrap().metadata()?.is_dir() {
            let json = Path::join(
                path.as_ref().unwrap().path().as_path(),
                Path::new("category.json"),
            );
            if json.exists() {
                valid_directories.push(path.unwrap().path().to_path_buf());
            }
        }
    }

    let mut tasks: Vec<data::Task> = vec![];

    for category in valid_directories.into_iter() {
        let path = Path::new(&category);
        let files = fs::read_dir(path)?;

        for file in files.into_iter() {
            if file.as_ref().unwrap().file_name().to_str().unwrap() != "category.json"
                && file.as_ref().unwrap().path().extension().unwrap() == "json"
            {
                let json_data = fs::read_to_string(file.unwrap().path())?;
                let data: data::Task = serde_json::from_str(&json_data)?;
                tasks.push(data);
            }
        }
    }

    Ok(tasks)
}
