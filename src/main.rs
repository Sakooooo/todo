use clap::{Args, Parser, Subcommand};

mod config;
mod handler;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::read_config()?;
    println!("{:?}", config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => handler::init::init(args, &mut config)?,
        Commands::List(args) => handler::list::list(args, &mut config)?,
    }

    Ok(())
}
