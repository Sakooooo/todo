use crate::{
    config,
    handler::data,
    helpers,
    helpers::styles::{BOLD, CATEGORY, FOLDER},
};
use std::{io, path::Path};

#[derive(Debug, clap::Args)]
pub struct RmArgs {
    directory: String,
    category: String,
    id: u64,
}

pub fn rm(
    args: &RmArgs,
    directory_config: &mut config::DirectoryConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_directory = helpers::get_directory(directory_config, args.directory.clone())?;

    let category = helpers::get_category(target_directory, args.category.clone())?;

    let todos = helpers::get_category_todos(category.clone())?;

    let mut target_task: Option<data::Task> = None;

    for todo in todos.into_iter() {
        if todo.id == args.id {
            target_task = Some(todo);
            break;
        }
    }

    let task = if let Some(value) = target_task {
        value
    } else {
        return Err(Box::new(helpers::errors::CommonErrors::TaskNotFound));
    };

    let filename = format!("{}.json", task.id);
    let path = Path::join(Path::new(&category), Path::new(&filename));

    let mut answer = String::new();
    println!(
        "{BOLD} Are you sure you want to remove task {} from {CATEGORY}{}{CATEGORY:#} in {FOLDER}{}{FOLDER:#}? {BOLD:#}",
        task.id, args.category, target_directory.name
    );
    io::stdin().read_line(&mut answer).unwrap();
    answer.pop();

    if answer.to_lowercase() == "yes" || answer.to_lowercase() == "y" {
        std::fs::remove_file(path)?;
        println!("Removed.")
    } else if answer.to_lowercase() == "no" || answer.to_lowercase() == "n" {
        println!("Aborting.");
    } else {
        println!("Unknown answer");
    }

    Ok(())
}
