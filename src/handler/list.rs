use crate::config;

#[derive(Debug, clap::Args)]
pub struct ListArgs {
    folder: Option<String>,
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
                println!("  {}", folder.name);
            }
        } else {
            println!("You have no folders!");
            println!("tip: Set one up with todo init ./todo");
        };
    };
    Ok(())
}
