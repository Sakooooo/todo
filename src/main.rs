use clap::{Args, Parser, Subcommand};

mod config;
mod handler;
mod helpers;

#[derive(Debug, Parser)]
#[clap(name = "todo", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initalize a task directory
    Init(handler::init::InitArgs),

    /// List directories or tasks in a directory
    List(handler::list::ListArgs),

    /// Add a task to directory
    Add(handler::add::AddArgs),

    /// Set a task's properties
    Set(handler::set::SetArgs),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::read_config()?;
    println!("{:?}", config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => handler::init::init(args, &mut config)?,
        Commands::List(args) => handler::list::list(args, &mut config)?,
        Commands::Add(args) => handler::add::new(args, &mut config)?,
        Commands::Set(args) => handler::set::set(args, &mut config)?,
    }

    Ok(())
}
