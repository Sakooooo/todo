use std::{fs::File, io::Write, path::Path};

use crate::{config, handler::data, helpers};

#[derive(Debug, clap::Args)]
pub struct SetArgs {
    directory: String,
    category: String,
    id: u64,

    #[arg(short, long)]
    task: Option<String>,

    #[arg(short, long)]
    state: Option<data::TaskState>,
}

pub fn set(args: &SetArgs, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let target_directory = helpers::get_directory(config, args.directory.clone())?;

    dbg!(&target_directory);

    let category = helpers::get_category(target_directory, args.category.clone())?;

    dbg!(&category);

    let todos = helpers::get_category_todos(category.clone())?;

    dbg!(&todos);

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

    dbg!(&target_task);

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
    dbg!(&updated_task_json);

    let mut file = std::fs::OpenOptions::new().write(true).open(output_path)?;

    file.write_all(updated_task_json.as_bytes())?;

    Ok(())
}
