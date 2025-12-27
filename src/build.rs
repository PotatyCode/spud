use std::process::Command;

use anyhow::{Context, Result};
pub fn build() -> Result<()> {
    let cmake_args = [".", "-B", "build"];
    Command::new("cmake")
        .args(cmake_args)
        .output()
        .context("Cmake failed to build")?;

    Command::new("cmake")
        .args(["--build", "build"])
        .status()
        .context("CMake build failed")?;
    Ok(())
}
