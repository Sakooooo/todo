use crate::{config, handler::data, helpers::errors::CommonErrors};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod errors;
pub mod styles;

pub const SCHEMA_VERSION: u64 = 1;

pub fn get_todos(
    directory: &config::Directory,
    category: String,
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

    for found_category in valid_directories.into_iter() {
        let path = Path::new(&found_category);
        let files = fs::read_dir(path)?;

        if found_category.file_name().unwrap().to_string_lossy() == category {
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
    }

    // This feels wrong...
    tasks.sort_by(|a, b| b.id.cmp(&a.id));
    tasks.reverse();

    Ok(tasks)
}

// todo need to make category struct to make this better
pub fn get_category_todos(
    category_path: String,
) -> Result<Vec<data::Task>, Box<dyn std::error::Error>> {
    let path = Path::new(&category_path);
    let files = fs::read_dir(path)?;

    let mut tasks: Vec<data::Task> = vec![];

    for file in files.into_iter() {
        if file.as_ref().unwrap().file_name().to_str().unwrap() != "category.json"
            && file.as_ref().unwrap().path().extension().unwrap() == "json"
        {
            let json_data = fs::read_to_string(file.unwrap().path())?;
            let data: data::Task = serde_json::from_str(&json_data)?;
            tasks.push(data);
        }
    }

    Ok(tasks)
}

pub fn get_directory(
    directory_config: &config::DirectoryConfig,
    target: String,
) -> Result<&config::Directory, Box<dyn std::error::Error>> {
    if directory_config.task_folder.is_none() {
        return Err(Box::new(CommonErrors::NoFolders));
    }

    let directories = directory_config.task_folder.as_ref().unwrap();

    let mut target_directory: Option<&config::Directory> = None;

    for directory in directories {
        if directory.name == target {
            target_directory = Some(directory);
        }
    }

    if target_directory.is_none() {
        return Err(Box::new(CommonErrors::FolderNotFound));
    }

    Ok(target_directory.unwrap())
}

pub fn get_category(
    directory: &config::Directory,
    target: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let directory_path = Path::new(&directory.path);

    let folders = fs::read_dir(directory_path)?;

    let mut result: Option<String> = None;

    for folder in folders.into_iter() {
        if folder.as_ref().unwrap().metadata()?.is_dir()
            && folder
                .as_ref()
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                == target
        {
            result = Some(folder.unwrap().path().to_string_lossy().to_string());
        };
    }

    if result.is_none() {
        return Err(Box::new(CommonErrors::CategoryNotFound));
    }

    Ok(result.unwrap())
}

pub fn get_categories(
    directory: &config::Directory,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let directory_path = Path::new(&directory.path);

    let folders = fs::read_dir(directory_path)?;

    let mut data: Vec<String> = vec![];

    for folder in folders.into_iter() {
        let json = Path::join(
            folder.as_ref().unwrap().path().as_path(),
            Path::new("category.json"),
        );

        if folder.as_ref().unwrap().metadata()?.is_dir() && json.exists() {
            let result = folder
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();
            data.push(result);
        };
    }

    Ok(data)
}
