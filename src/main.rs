use crate::{build::build, cli::Commands, config::Config};
use anyhow::{Context, Ok, Result};
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
            Ok(())
        }
        Commands::Build {} => {
            build().context("failed to build")?;
            Ok(())
        }
        Commands::Run { args } => {
            let config = Config::load("Spud.toml").context("failed to load Spud.toml")?;
            let spud_args: Vec<_> = args.iter().take_while(|&arg| arg != "--").skip(1).collect();
            let project_args: Vec<&String> =
                args.iter().skip_while(|&arg| arg != "--").skip(1).collect();
            build().context("failed to build")?;
            run::run(&config, project_args)?;
            Ok(())
        }
    }
}
