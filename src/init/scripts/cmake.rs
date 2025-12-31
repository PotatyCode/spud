use indoc::formatdoc;

use crate::config::Config;

pub fn cmake_template(config: &Config) -> String {
    formatdoc! {r#"
cmake_minimum_required (VERSION 4.2.1)
project({project_name} CXX) 

set(CMAKE_CXX_STANDARD, {cpp_standard})
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable({project_name} src/main.cpp)

target_compile_options({project_name} PRIVATE
  -Wall -Wextra -Wpedantic
  $<$<CONFIG:Debug>:-g>
  $<$<CONFIG:Release>:-03>
)
    "#, project_name = config.project.name,
    cpp_standard = config.project.cpp_standard}
}
