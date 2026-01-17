use anyhow::{self, Result};
use std::process::Command;
use walkdir::WalkDir;
#[cfg(test)]
mod tests {
    use std::fs::DirEntry;

    use super::*;
    #[test]
    fn check_fs() -> Result<(), anyhow::Error> {
        Command::new("spud").args(["init", "chip"]).status()?;
        let expected_dirs_and_files: Vec<&str> =
            vec!["include", "src", "CMakeLists.txt", "Spud.toml"];
        let mut dirs_and_files: Vec<&str> = vec![];
        for entry in WalkDir::new("chip") {
            let entry_str: String = *entry?.path().display().to_string().as_str();
            dirs_and_files.push(&entry_str);
        }
        assert!(expected_dirs_and_files == dirs_and_files);
        Ok(())
    }
}
