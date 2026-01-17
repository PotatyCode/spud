use indoc::formatdoc;
pub mod cmake;
pub mod spud_toml;
pub use cmake::cmake_template;
pub use spud_toml::spud_toml;
pub fn hello_world(proj_name: &str) -> String {
    formatdoc! {
        r#"
        #include<print>

        int main(){{
            std::println("hello, {proj_name}!"); //use this instead of std::cout <<
        }}
        "#
    }
}
