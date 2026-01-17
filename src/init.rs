use anyhow::{Context, Ok, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::config::Config;
pub mod scripts;
pub fn init_project(proj_name: &str) -> Result<()> {
    let root_dir = PathBuf::from(proj_name);
    fs::create_dir(proj_name)
        .with_context(|| format!("Failed to create directory {proj_name}",))?;
    create_toml(proj_name, &root_dir)?;
    let config = Config::load(&root_dir.join("Spud.toml")).context("failed to load Spud.toml")?;
    create_fs(&config, &root_dir)?;
    Ok(())
}
fn create_toml(proj_name: &str, root_dir: &Path) -> Result<()> {
    scripts::spud_toml(proj_name);
    File::write_all(
        &mut File::create_new(root_dir.join("Spud.toml"))?,
        scripts::spud_toml(proj_name).as_bytes(),
    )?;
    Ok(())
}
fn create_fs(config: &Config, root_dir: &Path) -> Result<()> {
    const SUB_DIRS: [&str; 2] = ["include", "src"];
    for dir in SUB_DIRS {
        let new_dir = root_dir.join(dir);
        fs::create_dir_all(&new_dir)
            .with_context(|| format!("Failed to create directory {dir}"))?;
        File::create_new(new_dir.join(".gitkeep"))?;
    }
    let mut hello_world = File::create_new(root_dir.join(SUB_DIRS[1]).join("main.cpp"))?;
    hello_world.write_all(scripts::hello_world(&config.project.name).as_bytes())?;

    let mut cmake = File::create_new(root_dir.join("CMakeLists.txt"))?;
    cmake.write_all(scripts::cmake_template(config).as_bytes())?;
    Ok(())
}
