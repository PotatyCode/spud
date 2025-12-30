use std::process::Command;

use crate::config::Config;
use anyhow::{Context, Result};
pub fn build(config: &Config) -> Result<()> {
    let cmake_args = [".", "-B", "build"];
    Command::new("cmake")
        .args(cmake_args)
        .output()
        .context("Cmake failed to build")?;

    Command::new("cmake")
        .args(["--build", "build"])
        .output()
        .context("CMake build failed")?;
    Ok(())
}
