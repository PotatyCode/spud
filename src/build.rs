use std::process::Command;

use crate::config::Config;
use anyhow::{Context, Result};
pub fn build(config: &Config) -> Result<()> {
    let cmake_args = [
        ".",
        "-B",
        "build",
        "-G",
        "Ninja",
        &format!("-DCMAKE_CXX_COMPILER={}", config.project.compiler),
    ];
    let output = Command::new("cmake")
        .args(cmake_args)
        .output()
        .context("Cmake failed to build")?;

    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

    let output = Command::new("cmake")
        .args(["--build", "build"])
        .output()
        .context("CMake build failed")?;
    // SHOW EVERYTHING
    println!("=== CMAKE BUILD ===");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}
