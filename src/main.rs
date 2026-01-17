use std::{fs::remove_dir_all, path::Path};

use crate::{build::build, cli::Commands, config::Config, run::execute};
use anyhow::{Context, Ok, Result, bail};
use clap::Parser;
pub mod build;
pub mod cli;
pub mod config;
pub mod init;
pub mod run;
pub use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { project_name } => {
            init::init_project(&project_name).context("Could init project")?;
        }
        Commands::Build(args) => {
            let config =
                Config::load(Path::new("Spud.toml")).context("failed to load Spud.toml")?;
            build(&args, &config).context("failed to build")?;
        }
        Commands::Run(args) => {
            let config =
                Config::load(Path::new("Spud.toml")).context("Failed to load Spud.toml")?;
            build(&args.build, &config)?;
            execute(args, &config)?;
        }
        Commands::Clean {} => {
            if Path::new("build").exists() {
                remove_dir_all("build")?
            } else {
                eprintln!("No build files to clean")
            }
        }
    }
    Ok(())
}
