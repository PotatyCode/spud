use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "spud")]
#[command(about = "Opinionated C++ project manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise project  
    Init { proj_name: String },
}
