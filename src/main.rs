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
    Init(handler::init::InitArgs, config::Config),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::read_config()?;
    println!("{:?}", config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args, config) => handler::init::init(args, config)?,
    }

    Ok(())
}
