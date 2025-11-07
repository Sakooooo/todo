use chrono::Local;
use notify_rust::Notification;

use crate::{
    config,
    handler::data::Task,
    helpers::{get_categories, get_todos, make_time_utc},
};

#[derive(Debug, clap::Args)]
pub struct WatchArgs {}

#[derive(Debug)]
struct WatchedTasks {
    name: String,
    category: String,
    todos: Vec<Task>,
}

pub fn watch(
    args: &WatchArgs,
    directory_config: &mut config::DirectoryConfig,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    dbg!(config);
    let mut monitored_tasks: Vec<WatchedTasks> = vec![];
    if let Some(directories) = &directory_config.task_folder {
        dbg!(directories);
        for directory in directories {
            let categories = get_categories(directory)?;
            for category in categories {
                let todos = get_todos(directory, Some(category.clone()))?;
                let data = WatchedTasks {
                    name: directory.name.clone(),
                    category,
                    todos,
                };
                monitored_tasks.push(data);
            }
        }
    };

    for task in monitored_tasks {
        for todo in task.todos {
            if let Some(schedule) = todo.scheduled {
                dbg!(schedule);
                if schedule.time().to_string() != "00:00:00" {
                    let distance = Local::now() - schedule.with_timezone(&Local);
                    dbg!(schedule);
                    dbg!(distance);
                };
            }
        }
    }
    loop {
        println!("Test");
        std::thread::sleep_ms(2000);
        Notification::new()
            .summary("This is a test.")
            .body("This is just a test.")
            .appname("todo")
            .show()?;
    }
}
