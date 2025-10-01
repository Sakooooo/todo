use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::config;

#[derive(Debug, clap::Args)]
pub struct ListArgs {
    folder: Option<String>,
}

fn get_todos(directory: &config::Directory) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", directory.name);

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

    for category in valid_directories.into_iter() {
        let path = Path::new(&category);
        dbg!(path);
        let files = fs::read_dir(path)?;
        dbg!(files);

        // for file in files.into_iter() {}
    }

    Ok(())
}

pub fn list(
    args: &ListArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.folder.is_some() {
    } else {
        if config.task_folder.is_some() {
            println!("Folders:");
            for folder in config.task_folder.as_ref().unwrap() {
                get_todos(folder)?;
            }
        } else {
            println!("You have no folders!");
            println!("tip: Set one up with todo init ./todo");
        };
    };
    Ok(())
}
