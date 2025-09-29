use std::path::Path;

use crate::config;

#[derive(Debug, clap::Args)]
pub struct InitArgs {
    directory: String,
}

pub fn init(args: &InitArgs, config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&args.directory.clone()).is_dir() {
        std::fs::create_dir_all(args.directory.clone())?;
    };
    println!("{:?}", args);
    Ok(())
}
