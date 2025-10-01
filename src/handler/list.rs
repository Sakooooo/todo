use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{config, handler::data};

#[derive(Debug, clap::Args)]
pub struct ListArgs {
    folder: Option<String>,
}

#[derive(Debug)]
enum ListErrors {
    DoesntExist,
    NoFolders,
}

impl std::fmt::Display for ListErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoesntExist => write!(f, "Folder does not exist!"),
            Self::NoFolders => write!(f, "Folder does not exist!"),
        }
    }
}

impl std::error::Error for ListErrors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

fn get_todos(directory: &config::Directory) -> Result<Vec<data::Task>, Box<dyn std::error::Error>> {
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

pub fn list(
    args: &ListArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.folder.is_some() {
        let mut directory: Option<&config::Directory> = None;

        for folder in config.task_folder.as_ref().unwrap() {
            if &folder.name == args.folder.as_ref().unwrap() {
                directory = Some(folder);
                break;
            };
        }

        if directory.is_none() {
            return Err(Box::new(ListErrors::DoesntExist));
        }

        let todos = get_todos(directory.unwrap())?;

        println!("{}", directory.unwrap().name);
        for todo in todos.into_iter() {
            println!("  {} {} {}", todo.id, todo.state, todo.task);
        }
    } else {
        if config.task_folder.is_some() {
            println!("Folders:");
            for folder in config.task_folder.as_ref().unwrap() {
                println!("  {}", folder.name);
                let todos = get_todos(folder)?;
                for todo in todos.into_iter() {
                    println!("      {} {} {}", todo.id, todo.state, todo.task);
                }
            }
        } else {
            println!("You have no folders!");
            println!("tip: Set one up with todo init ./todo");
            return Err(Box::new(ListErrors::NoFolders));
        };
    };
    Ok(())
}
