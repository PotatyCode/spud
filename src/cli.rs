use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spud")]
#[command(about = "Opinionated C++ project manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]

pub enum Commands {
    /// Initilise project
    Init { project_name: String },

    /// Build project
    Build(BuildArgs),
    /// Builds and Runs project
    Run(RunArgs),
    /// Cleans build files
    Clean {},
}
// Shared build configuration
//
#[derive(Args, Clone, Debug)]
pub struct BuildArgs {
    #[arg(short, long)]
    pub release: bool,

    #[arg(short, long)]
    pub target: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct RunArgs {
    #[command(flatten)]
    pub build: BuildArgs,

    #[arg(last = true)]
    pub program_args: Vec<String>,
}
