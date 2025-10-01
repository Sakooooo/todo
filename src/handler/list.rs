use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{config, handler::data, helpers};

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

        let todos = helpers::get_todos(directory.unwrap())?;

        println!("{}", directory.unwrap().name);
        for todo in todos.into_iter() {
            println!("  {} {} {}", todo.id, todo.state, todo.task);
        }
    } else {
        if config.task_folder.is_some() {
            println!("Folders:");
            for folder in config.task_folder.as_ref().unwrap() {
                println!("  {}", folder.name);
                let todos = helpers::get_todos(folder)?;
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
