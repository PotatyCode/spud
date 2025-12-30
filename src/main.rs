use crate::{build::build, cli::Commands, config::Config};
use anyhow::{Context, Ok, Result};
use clap::Parser;
pub mod build;
pub mod cli;
pub mod config;
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
        Commands::Build {} => {
            let config = Config::load("Spud.toml").context("failed to load Spud.toml")?;
            build(&config).context("failed to build")?;
            build().context("failed to build")?;
            Ok(())
        }
        Commands::Run { args } => {
            let config = Config::load("Spud.toml")?;
            let (spud_args, project_args) = run::split_args(args);
            build().context("failed to build")?;
            run::run(&config, project_args)?;
            Ok(())
        }
    }
}
