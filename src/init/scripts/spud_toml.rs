use indoc::formatdoc;

pub fn spud_toml(proj_name: &str) -> String {
    formatdoc! {
        r#"
        [project]
        name = "{proj_name}"
        cpp_standard = 23
        compiler = "clang++"
        "#
    }
}
