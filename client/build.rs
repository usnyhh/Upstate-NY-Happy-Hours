//! Compiles `ui/app.slint` into Rust (included in `src/app.rs` via `slint::include_modules!`).

fn main() {
    // Pin the widget style so web and desktop look the same (otherwise desktop
    // picks a platform-native style), and use its dark variant.
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/app.slint", config).expect("failed to compile ui/app.slint");
}
