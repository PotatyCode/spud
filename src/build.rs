use crate::{cli::BuildArgs, config::Config};
use anyhow::{Result, bail};
use std::process::Command;
pub fn build(args: &BuildArgs, config: &Config) -> Result<()> {
    let compiler_arg = &format!("-DCMAKE_CXX_COMPILER={}", config.project.compiler);
    let mut cmake_args: Vec<&str> = vec!["-B", "build", "-G", "Ninja", compiler_arg];
    if args.release {
        cmake_args.push("-DCMAKE_BUILD_TYPE=Release");
    }
    let output = Command::new("cmake").args(cmake_args).output()?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("CMake configuration failed:\n{}{}", stdout, stderr);
    }

    let output = Command::new("cmake")
        .args(["--build", "build", "--parallel"])
        .output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!("\n{}", stdout);
    }
    Ok(())
}
