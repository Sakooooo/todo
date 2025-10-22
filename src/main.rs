use clap::{Parser, Subcommand};

mod config;
mod handler;
mod helpers;

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

#[derive(Debug, Parser)]
#[clap(name = "todo", version)]
#[command(styles=get_styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initalize a task directory
    Init(handler::init::InitArgs),

    /// List directories or tasks in a directory
    #[clap(alias = "ls")]
    List(handler::list::ListArgs),

    /// Add a task to directory
    Add(handler::add::AddArgs),

    /// Set a task's properties
    Set(handler::set::SetArgs),

    /// Remove a task
    Rm(handler::rm::RmArgs),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut directory_config = config::read_directory_config()?;
    // println!("{:?}", config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(args) => handler::init::init(args, &mut directory_config)?,
        Commands::List(args) => handler::list::list(args, &mut directory_config)?,
        Commands::Add(args) => handler::add::new(args, &mut directory_config)?,
        Commands::Set(args) => handler::set::set(args, &mut directory_config)?,
        Commands::Rm(args) => handler::rm::rm(args, &mut directory_config)?,
    }

    Ok(())
}
