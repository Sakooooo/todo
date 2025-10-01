use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::{config, handler::data};

#[derive(Debug, clap::Args)]
pub struct NewArgs {
    directory: String,

    #[arg(short, long)]
    category: Option<String>,

    #[arg(short, long)]
    task: String,

    #[arg(short, long)]
    #[clap(default_value_t, value_enum)]
    status: data::TaskState,
}

#[derive(Debug)]
enum NewError {
    NoDirectories,
    NotFound,
}

impl std::fmt::Display for NewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoDirectories => write!(f, "There are no directories to add to!"),
            Self::NotFound => write!(f, "Category not found!"),
        }
    }
}

impl std::error::Error for NewError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

pub fn new(args: &NewArgs, config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let target: String;

    if args.category.is_none() {
        target = "inbox".to_string();
    } else {
        target = args.category.clone().unwrap();
    }

    if config.task_folder.is_none() {
        return Err(Box::new(NewError::NoDirectories));
    }

    let directories = config.task_folder.as_ref().unwrap();

    let mut result: Option<&config::Directory> = None;

    for directory in directories.into_iter() {
        if directory.name == args.directory {
            result = Some(directory);
            break;
        }
    }

    let target_directory: &config::Directory;

    if result.is_none() {
        return Err(Box::new(NewError::NotFound));
    } else {
        target_directory = result.unwrap();
    }

    let directory_path = Path::new(&target_directory.path);
    let paths = fs::read_dir(directory_path)?;

    let mut category_path: Option<&Path> = None;
    let mut category_exists = false;
    let mut category_json_exists = false;

    let mut unwrapped: std::path::PathBuf;
    let mut category_info_path_joined: std::path::PathBuf;

    for path in paths {
        unwrapped = path.unwrap().path();

        category_info_path_joined = Path::join(unwrapped.as_path(), Path::new("category.json"));

        if unwrapped.is_dir() {
            if unwrapped.file_name().unwrap().to_string_lossy() == target {
                // category_path = Some(&unwrapped.as_path());
                let category_info_path: Option<&Path> = Some(category_info_path_joined.as_path());
                if category_info_path.unwrap().exists() {
                    category_exists = true;
                    category_json_exists = true;
                };
                category_path = Some(&unwrapped);
                break;
            }
        }
    }

    let full_path: String;
    if !category_exists {
        full_path = format!("{}/{}", directory_path.display(), target).to_string();
        category_path = Some(&Path::new(&full_path));
        fs::create_dir(category_path.unwrap())?;
    };

    let category_info_filepath = Path::join(category_path.unwrap(), Path::new("category.json"));

    if !category_json_exists {
        let mut info = data::CategoryInfo {
            id: 0,
            latest_todo_id: 0,
        };

        let mut highest_id: u64 = 0;
        let mut _category_info: Option<data::CategoryInfo> = None;

        for folder in config.task_folder.as_ref().unwrap() {
            let path = format!("{}/category.json", folder.path);
            let category_path = Path::new(&path);
            if category_path.exists() {
                let category_data = fs::read_to_string(&category_path)?;
                _category_info = serde_json::from_str(&category_data)?;

                if _category_info.as_ref().unwrap().id > highest_id {
                    highest_id = _category_info.as_ref().unwrap().id;
                }
            };
        }

        info.id += highest_id;

        let info_json = serde_json::to_string_pretty(&info)?;

        let mut category_info_file = File::create(&category_info_filepath)?;

        category_info_file.write_all(&info_json.into_bytes())?;
    }

    let category_info_content = fs::read_to_string(&category_info_filepath)?;
    let mut category_info_result: data::CategoryInfo =
        serde_json::from_str(&category_info_content)?;

    let task = data::Task {
        id: category_info_result.latest_todo_id + 1,
        state: args.status.clone(),
        task: args.task.clone(),
    };
    let task_json = serde_json::to_string_pretty(&task.clone())?;

    let filename = format!("{}.json", task.id);
    let task_filename = Path::new(&filename);
    let task_path = Path::join(category_path.unwrap(), task_filename);

    let mut task_file = File::create(task_path)?;
    task_file.write_all(&task_json.into_bytes())?;

    category_info_result.latest_todo_id += 1;

    let mut category_info_file = File::create(category_info_filepath)?;
    let category_info_json = serde_json::to_string_pretty(&category_info_result)?;
    category_info_file.write_all(&category_info_json.into_bytes())?;

    Ok(())
}
