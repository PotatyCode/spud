use anyhow::{Context, Ok, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
mod scripts;
pub fn init_project(proj_name: &str) -> Result<()> {
    create_fs(proj_name)?;
    Ok(())
}

fn create_fs(proj_name: &str) -> Result<()> {
    let proj = PathBuf::from(proj_name);
    fs::create_dir(&proj).with_context(|| format!("Failed to create directory {proj_name}"))?;

    const SUB_DIRS: [&str; 2] = ["include", "src"];
    for dir in SUB_DIRS {
        let new_dir = proj.join(dir);
        fs::create_dir_all(&new_dir)
            .with_context(|| format!("Failed to create directory {dir}"))?;
        File::create_new(new_dir.join(".gitkeep"))?;
    }
    let mut hello_world = File::create_new(proj.join(SUB_DIRS[1]).join("main.cpp"))?;
    hello_world.write_all(scripts::hello_world(proj_name).as_bytes())?;

    let mut cmake = File::create_new(proj.join("CMakeLists.txt"))?;
    cmake.write_all(scripts::cmake::cmake(proj_name).as_bytes())?;

    let mut toml = File::create_new(proj.join("Spud.toml"))?;
    toml.write_all(scripts::spud_toml::spud_toml(proj_name).as_bytes())?;
    Ok(())
}
