use anyhow::{Context, Ok, Result};
use std::process::Command;

use crate::config::Config;
pub fn run(config: &Config, args: Vec<String>) -> Result<()> {
    Command::new(format!("./build/{}", config.project.name))
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("cannot run executable build/")?;
    Ok(())
}

pub fn split_args(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let split_pos = args.iter().position(|arg| arg == "::");
    match split_pos {
        Some(pos) => (args[..pos].to_vec(), args[pos + 1..].to_vec()),
        None => (args, vec![]),
    }
}
