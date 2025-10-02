use std::{
    io::{self, Write},
    path::Path,
};

use crate::{
    config::{self, save_config},
    handler::data,
};

#[derive(Debug, clap::Args)]
pub struct InitArgs {
    directory: String,
}

#[derive(Debug)]
enum InitError {
    AlreadyExists,
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AlreadyExists => write!(f, "Folder already exists!"),
        }
    }
}

impl std::error::Error for InitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn init(
    args: &InitArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = Path::new(&args.directory);

    if !filepath.is_dir() && !filepath.try_exists().unwrap() {
        std::fs::create_dir_all(args.directory.clone())?;
    };

    let todoinfo = Path::join(filepath, Path::new("todo.json"));

    if todoinfo.exists() {
        return Err(Box::new(InitError::AlreadyExists));
    }

    let path = std::fs::canonicalize(filepath)?
        .into_os_string()
        .into_string()
        .unwrap();

    let mut name = String::new();

    println!("Name the task folder:");
    io::stdin().read_line(&mut name).unwrap();
    name.pop();

    let directory = config::Directory {
        name: name.clone(),
        path: path.clone(),
    };

    let info = data::DirectoryInfo { name: name.clone() };
    let info_json = serde_json::to_string_pretty(&info)?;

    if config.task_folder.is_some() {
        config.task_folder.as_mut().unwrap().push(directory);
    } else {
        config.task_folder = Some(vec![directory]);
    }

    let mut directory_info = std::fs::File::create(todoinfo)?;

    directory_info.write_all(&info_json.into_bytes())?;

    save_config(config)?;

    println!("Created task folder {} at {}", name, path);

    Ok(())
}
