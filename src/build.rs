use std::process::Command;

use anyhow::{Context, Result};

use crate::config::Config;
pub fn build(_config: &Config) -> Result<()> {
    let cmake_args = ["-B", "build", "-G", "Ninja", "-DCMAKE_CXX_COMPILER=clang++"];
    let build = Command::new("cmake")
        .args(cmake_args)
        .status()
        .context("Cmake failed to build")?;

    if build.success() {
        let _comp = Command::new("cmake").arg("--build").arg("build").status()?;
    }
    Ok(())
}
