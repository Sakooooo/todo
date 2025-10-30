use chrono::Local;

use crate::{
    config,
    handler::data::Task,
    helpers::{self, styles::*},
};
#[derive(Debug, clap::Args)]
pub struct ListArgs {
    /// The folder to list
    folder: Option<String>,
    /// The category to list
    category: Option<String>,
    /// The amount of tasks to list in a folder
    #[arg(short, long, default_value_t = 10)]
    count: u64,
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
        None
    }
}

fn output_list(
    todos: Vec<Task>,
    count: usize,
    category: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let targets = if count > todos.len() {
        0
    } else {
        todos.len() - count
    };
    let selected_todos = &todos[targets..todos.len()];
    println!("  {BOLD}{}{BOLD:#}", category);
    for todo in selected_todos.iter() {
        print!("      {} {} {}", todo.id, todo.state, todo.task);
        if let Some(deadline) = todo.deadline {
            print!(
                " {BOLD}{DEADLINE}DEADLINE:{DEADLINE:#}{BOLD:#} {}",
                deadline.date_naive()
            );
            if deadline.time().format("%H:%M").to_string() != "00:00" {
                print!(
                    ", {}",
                    deadline.with_timezone(&Local).time().format("%H:%M")
                )
            }
        };
        if let Some(scheduled) = todo.scheduled {
            print!(
                " {BOLD}{SCHEDULED}SCHEDULED:{SCHEDULED:#}{BOLD:#} {}",
                scheduled.date_naive()
            );
            if scheduled.time().format("%H:%M").to_string() != "00:00" {
                print!(", {}", scheduled.with_timezone(&Local).time())
            }
        };
        println!();
    }
    Ok(())
}

pub fn list(
    args: &ListArgs,
    directory_config: &mut config::DirectoryConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if args.folder.is_some() {
        let mut directory: Option<&config::Directory> = None;

        for folder in directory_config.task_folder.as_ref().unwrap() {
            if &folder.name == args.folder.as_ref().unwrap() {
                directory = Some(folder);
                break;
            };
        }

        if directory.is_none() {
            return Err(Box::new(ListErrors::DoesntExist));
        }

        println!("{BOLD}{}{BOLD:#}", directory.unwrap().name);
        let categories = helpers::get_categories(directory.unwrap())?;
        for category in categories.into_iter() {
            let todos = helpers::get_todos(directory.unwrap(), category.clone())?;
            output_list(todos, args.count as usize, category)?;
        }
    } else if directory_config.task_folder.is_some() {
        println!("{BOLD}Folders:{BOLD:#}");
        for folder in directory_config.task_folder.as_ref().unwrap() {
            println!("  {BOLD}{}{BOLD:#}", folder.name);
            let categories = helpers::get_categories(folder)?;
            for category in categories.into_iter() {
                let todos = helpers::get_todos(folder, category.clone())?;
                output_list(todos, args.count as usize, category)?;
            }
        }
    } else {
        println!("You have no folders!");
        println!("tip: Set one up with todo init ./todo");
        return Err(Box::new(ListErrors::NoFolders));
    };
    Ok(())
}
