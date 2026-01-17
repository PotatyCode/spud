use crate::config::Config;
use anyhow::{Context, Result};
use std::process::Command;
pub fn build(config: &Config) -> Result<()> {
    let cmake_args = [
        "-B",
        "build",
        "-G",
        "Ninja",
        &format!("-DCMAKE_CXX_COMPILER={}", config.project.compiler),
    ];
    let build = Command::new("cmake")
        .args(cmake_args)
        .status()
        .context("Cmake failed to build")?;

    if build.success() {
        let _comp = Command::new("cmake")
            .args(["--build", "build", "--parallel"])
            .status()?;
    }
    Ok(())
}
