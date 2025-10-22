use std::{io::Write, path::Path};

use crate::{config, handler::data, helpers, helpers::styles::*};

#[derive(Debug, clap::Args)]
pub struct SetArgs {
    directory: String,
    category: String,
    id: u64,

    #[arg(short, long)]
    // Task to change the original task to
    task: Option<String>,

    #[arg(short, long)]
    // State to change task to
    state: Option<data::TaskState>,
}

pub fn set(
    args: &SetArgs,
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
        };
    }

    if target_task.is_none() {
        return Err(Box::new(helpers::errors::CommonErrors::TaskNotFound));
    }

    let mut task = target_task.unwrap();

    if args.state.is_some() {
        task.state = args.state.clone().unwrap();
    }

    if args.task.is_some() {
        task.task = args.task.clone().unwrap();
    }

    let filename = format!("{}.json", task.id);
    let output_path = Path::join(Path::new(&category), Path::new(&filename));

    let updated_task_json = serde_json::to_string_pretty(&task)?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(output_path)?;

    file.write_all(updated_task_json.as_bytes())?;

    println!(
        "Updated item ID {BOLD}{}{BOLD:#} in category {CATEGORY}{}{CATEGORY:#} of folder {FOLDER}{}{FOLDER:#}.",
        task.id, args.category, args.directory,
    );

    Ok(())
}
