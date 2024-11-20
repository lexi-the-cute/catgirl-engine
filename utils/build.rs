//! Build script for crate

use build_info_build::DependencyDepth;
use std::env;

/// Main function
fn main() {
    // Generate build info
    generate_build_info();
}

/// Checks if string matches environment variable
fn matches_environment_var(key: &str, value: &str) -> bool {
    let environment_var: Result<String, env::VarError> = env::var(key);
    environment_var.is_ok() && environment_var.unwrap() == value
}

/// Generate build info
fn generate_build_info() {
    let mut depth: DependencyDepth = DependencyDepth::Depth(0);

    // Track environment for rebuilds
    println!("cargo:rerun-if-env-changed=RUST_ANALYZER");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    // Custom environment variable to speed up writing code
    let rust_analyzer: bool = matches_environment_var("RUST_ANALYZER", "true");
    let docs_rs: bool = env::var("DOCS_RS").is_ok();
    if rust_analyzer || docs_rs {
        depth = DependencyDepth::None;
    }

    build_info_build::build_script().collect_runtime_dependencies(depth);
}
