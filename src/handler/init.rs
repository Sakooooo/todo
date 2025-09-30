use std::{io, path::Path};

use crate::{
    config::{self, save_config},
    handler::data,
};

#[derive(Debug, clap::Args)]
pub struct InitArgs {
    directory: String,
}

pub fn init(
    args: &InitArgs,
    config: &mut config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = Path::new(&args.directory);
    if !filepath.is_dir() {
        std::fs::create_dir_all(args.directory.clone())?;
    };

    let path = std::fs::canonicalize(filepath)?
        .into_os_string()
        .into_string()
        .unwrap();

    let mut name = String::new();

    println!("Name the task folder:");
    io::stdin().read_line(&mut name).unwrap();
    name.pop();

    let info = data::Directory { name, path };

    println!("appending {:?}", info);

    if config.task_folder.is_some() {
        config.task_folder.as_mut().unwrap().push(info);
    } else {
        config.task_folder = Some(vec![info]);
    }

    println!("{:?}", config);

    println!("{:?}", args);

    save_config(config)?;

    Ok(())
}
