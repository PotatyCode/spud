use crate::cli::Commands;
use anyhow::{Context, Ok, Result};
use clap::{Command, Parser};
pub mod build;
pub mod cli;
pub mod init;
pub use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { proj_name } => {
            init::init_project(&proj_name).context("Could init project")?;
            Ok(())
        }
    }
}
