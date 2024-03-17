#[cfg(feature = "build-info")]
use build_info_build::DependencyDepth;
use std::env;

fn main() {
    // Generate build info
    #[cfg(feature = "build-info")]
    generate_build_info();
}

#[allow(dead_code)]
fn matches_environment_var(key: &str, value: &str) -> bool {
    let environment_var: Result<String, env::VarError> = env::var(key);
    environment_var.is_ok() && environment_var.unwrap() == value
}

#[cfg(feature = "build-info")]
fn generate_build_info() {
    let mut depth: DependencyDepth = DependencyDepth::Depth(0);

    // Custom environment variable to speed up writing code
    let rust_analyzer: bool = matches_environment_var("RUST_ANALYZER", "true");
    let docs_rs: bool = env::var("DOCS_RS").is_ok();
    if rust_analyzer || docs_rs {
        depth = DependencyDepth::None;
    }

    let options: build_info_build::BuildScriptOptions =
        build_info_build::build_script().collect_runtime_dependencies(depth);
    if docs_rs {
        options.set_offline(true);
    }
}
