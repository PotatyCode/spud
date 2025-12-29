use anyhow::{Context, Ok, Result};
use std::process::Command;

use crate::config::Config;
pub fn run(config: &Config, args: Vec<&String>) -> Result<()> {
    Command::new(format!("./build/{}", config.project.name))
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("cannot run executable build/")?;
    Ok(())
}
