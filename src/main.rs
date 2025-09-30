use clap::{Args, Parser, Subcommand};

mod config;
mod handler;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initalize a task directory
    Init(handler::init::InitArgs),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::read_config()?;
    println!("{:?}", config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => handler::init::init(args, &mut config)?,
    }

    Ok(())
}
