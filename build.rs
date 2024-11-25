//! Build script for crate

extern crate cbindgen;

use build_info_build::DependencyDepth;
use cbindgen::{Config, CythonConfig, Language};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

/// Main function
fn main() {
    // Debug environment
    // print_environment_vars();

    // Generate build info
    generate_build_info();

    // Bindings are only usable when building libs
    create_bindings();
}

/// Generate build info
fn generate_build_info() {
    // https://github.com/danielschemmel/build-info/issues/17
    // https://github.com/danielschemmel/build-info/issues/18
    let mut depth: DependencyDepth = DependencyDepth::Depth(0);

    // Track environment for rebuilds
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
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

/// Create C/C++/Python bindings
fn create_bindings() {
    let crate_directory: PathBuf = crate_dir();
    let package_name: String = env::var("CARGO_PKG_NAME").unwrap();

    create_binding("h", Language::C, &package_name, &crate_directory);
    create_binding("hpp", Language::Cxx, &package_name, &crate_directory);
    create_binding(
        "pxd",
        Language::Cython,
        &package_name.replace('-', "_"),
        &crate_directory,
    );
}

/// Create requested binding
fn create_binding(
    extension: &str,
    language: Language,
    package_name: &String,
    crate_directory: &PathBuf,
) {
    let output_file: String = target_dir()
        .join("binding")
        .join(format!("{package_name}.{extension}"))
        .display()
        .to_string();

    let mut header: String = String::new() +
        "/*\n" +
        " * This file exists to help facilitate modding this catgirl game engine...\n" +
        " * These generated bindings are either public domain or Unlicense where public domain does not exist\n" +
        " */";
    if language == Language::Cython {
        header =
            "# cython: language_level=3\n\n".to_string() +
            "# This file exists to help facilitate modding this catgirl game engine...\n" +
            "# These generated bindings are either public domain or Unlicense where public domain does not exist";
    }

    let defines: HashMap<String, String> = get_bindgen_defines();

    // Ensures including the workspace crates
    let workspace_crates = vec![
        format!("{package_name}-client"),
        format!("{package_name}-server"),
        format!("{package_name}-utils"),
    ];
    let parse_config: cbindgen::ParseConfig = cbindgen::ParseConfig {
        parse_deps: true,
        include: Some(workspace_crates.clone()),
        extra_bindings: workspace_crates,
        ..Default::default()
    };

    let mut config: Config = cbindgen::Config {
        namespace: Some(String::from("ffi")),
        header: Some(header),
        only_target_dependencies: true,
        no_includes: language == Language::Cython,
        parse: parse_config,
        language,
        defines,
        ..Default::default()
    };

    if language == Language::Cython {
        let crate_name: String = env::var("CARGO_PKG_NAME").unwrap();
        let header_filename: String = format!("\"<{}.h>\"", crate_name);

        config.cython = CythonConfig {
            header: Some(header_filename),
            ..Default::default()
        };
    }

    cbindgen::generate_with_config(crate_directory, config)
        .unwrap()
        .write_to_file(output_file);
}

/// Define custom C defines macros
fn get_bindgen_defines() -> HashMap<String, String> {
    let mut defines: HashMap<String, String> = HashMap::new();

    // Features
    defines.insert(
        "feature = client".to_string(),
        "DEFINE_CLIENT_FEATURE".to_string(),
    );
    defines.insert(
        "feature = server".to_string(),
        "DEFINE_SERVER_FEATURE".to_string(),
    );

    // Basic OS Targets
    defines.insert(
        "target_os = android".to_string(),
        "DEFINE_ANDROID_OS".to_string(),
    );
    defines.insert(
        "target_os = windows".to_string(),
        "DEFINE_WINDOWS_OS".to_string(),
    );
    defines.insert(
        "target_os = macos".to_string(),
        "DEFINE_MACOS_OS".to_string(),
    );
    defines.insert("target_os = ios".to_string(), "DEFINE_IOS_OS".to_string());
    defines.insert(
        "target_os = linux".to_string(),
        "DEFINE_LINUX_OS".to_string(),
    );

    // Basic Family Targets
    defines.insert(
        "target_family = unix".to_string(),
        "DEFINE_UNIX_FAMILY".to_string(),
    );
    defines.insert(
        "target_family = windows".to_string(),
        "DEFINE_WINDOWS_FAMILY".to_string(),
    );

    defines
}

/// Find the location of the project's root directory
fn crate_dir() -> PathBuf {
    PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        crate_dir().join("target")
    }
}

/// Print all environment variables
#[allow(dead_code)]
fn print_environment_vars() {
    let vars: env::Vars = env::vars();

    for (key, var) in vars {
        println!("cargo:warning=EV: {key}: {var}");
    }
}
