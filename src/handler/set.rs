use crate::{config, handler::data, helpers};

#[derive(Debug, clap::Args)]
pub struct SetArgs {
    directory: String,
    category: String,
    id: u64,
    state: Option<data::TaskState>,
    task: Option<String>,
}

// todo
// helper func for getting categories
pub fn set(args: &SetArgs, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
    if config.task_folder.is_none() {
        return Err(Box::new(helpers::errors::CommonErrors::NoFolders));
    }

    let directories = config.task_folder.as_ref().unwrap();

    let mut target_directory: Option<&config::Directory> = None;

    for directory in directories {
        if directory.name == args.directory {
            target_directory = Some(directory);
            break;
        }
    }

    if target_directory.is_none() {
        return Err(Box::new(helpers::errors::CommonErrors::FolderNotFound));
    }

    let todos = helpers::get_todos(target_directory.unwrap())?;

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

    Ok(())
}
