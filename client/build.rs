//! Build script for crate

use build_info_build::DependencyDepth;
use std::env;

/// Main function
fn main() {
    // Generate build info
    generate_build_info();
}

/// Generate build info
fn generate_build_info() {
    let mut depth: DependencyDepth = DependencyDepth::Depth(0);

    // Track environment for rebuilds
    println!("cargo:rerun-if-env-changed=RUST_ANALYZER");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    // Custom environment variable to speed up writing code
    let rust_analyzer: bool = env::var("RUST_ANALYZER").is_ok();
    let docs_rs: bool = env::var("DOCS_RS").is_ok();
    if rust_analyzer || docs_rs {
        depth = DependencyDepth::None;
    }

    build_info_build::build_script().collect_runtime_dependencies(depth);
}
