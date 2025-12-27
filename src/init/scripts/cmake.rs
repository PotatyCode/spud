use indoc::formatdoc;

pub fn cmake(proj_name: &str) -> String {
    formatdoc! {r#"
cmake_minimum_required (VERSION 4.2.1)
project({proj_name})
add_executable({proj_name} src/main.cpp)
    "#}
}
