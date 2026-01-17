use indoc::formatdoc;

use crate::config::Config;

pub fn cmake_template(config: &Config) -> String {
    formatdoc! {r#"
cmake_minimum_required (VERSION 4.2.1)
project({proj_name}) 

set(CMAKE_CXX_STANDARD {standard})
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable({proj_name} src/main.cpp)
    "#, proj_name = config.project.name, standard = config.project.cpp_standard}
}
