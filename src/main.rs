use std::{fs::remove_dir_all, path::Path};

use crate::{build::build, cli::Commands, config::Config};
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
        Commands::Init { proj_name } => {
            init::init_project(&proj_name).context("Could init project")?;
        }
        Commands::Build {} => {
            let config =
                Config::load(Path::new("Spud.toml")).context("failed to load Spud.toml")?;
            build(&config).context("failed to build")?;
        }
        Commands::Run { args } => {
            let config =
                Config::load(Path::new("Spud.toml")).context("Failed to load Spud.toml")?;
            let (_spud_args, project_args) = run::split_args(args);
            build(&config).context("failed to build")?;
            run::run(&config, project_args)?;
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
