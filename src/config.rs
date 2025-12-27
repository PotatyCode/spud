use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use std::fs::read_to_string;
#[derive(Deserialize, Debug)]
pub struct Config {
    pub project: Project,
}
#[derive(Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub cpp_standard: u8,
    pub compiler: String,
}
impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let contents = read_to_string(path).context("failed to read toml file")?;
        Ok(toml::from_str(&contents)?)
    }
}
