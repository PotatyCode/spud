use std::path::Path;

use anyhow::{Context, Ok, Result};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "example"]
struct Templates;
pub fn init_project(proj_name: &str) -> Result<()> {
    for file_path in Templates::iter() {
        let content = Templates::get(&file_path).unwrap();
        let original = std::str::from_utf8(&content.data).unwrap();
        let rendered = original.replace("{{project_name}}", proj_name);
        let new_path = file_path.replace("__project__", proj_name);
        let full_path = Path::new(proj_name).join(new_path);
        std::fs::create_dir_all(full_path.parent().unwrap())?;
        std::fs::write(full_path, rendered)?;
    }
    Ok(())
}
