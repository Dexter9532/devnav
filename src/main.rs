mod config;
mod init;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Configure where DevNav finds projects
    Init,
    /// Update DevNav to the latest version
    Update,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => init::run(),
        Command::Update => update::run(),
    }
}
