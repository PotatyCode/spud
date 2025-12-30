use indoc::formatdoc;

use crate::config::Config;

pub fn cmake_template(config: &Config) -> String {
    formatdoc! {r#"
cmake_minimum_required (VERSION 4.2.1)
project({project_name}) 
add_executable({project_name} src/main.cpp)
    "#, project_name = config.project.name}
}
