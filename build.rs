//! Build script for crate

/// Main function
fn main() {
    // Tell Cargo where the resources directory is located
    set_environment_variables();

    // Debug environment
    // print_environment_vars();

    // Generate build info
    generate_build_info();

    // Bindings are only usable when building libs
    create_bindings();
}

/// Sets environment variables for building
fn set_environment_variables() {
    let resources_path: std::path::PathBuf = crate_dir().join("resources");
    let resources_path_str: &str = resources_path.to_str().unwrap();

    // println!("cargo:warning=Setting resources path to {resources_path_str}");
    println!("cargo:rustc-env=ENGINE_RESOURCES_PATH={resources_path_str}");
}

/// Generate build info
fn generate_build_info() {
    // https://github.com/danielschemmel/build-info/issues/17
    // https://github.com/danielschemmel/build-info/issues/18
    let depth: build_info_build::DependencyDepth = build_info_build::DependencyDepth::Depth(0);

    // Track environment for rebuilds
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo:rerun-if-env-changed=RUST_ANALYZER");
    println!("cargo:rerun-if-env-changed=DOCS_RS");

    // Custom environment variable to speed up writing code
    let rust_analyzer: bool = std::env::var("RUST_ANALYZER").is_ok();
    let docs_rs: bool = std::env::var("DOCS_RS").is_ok();

    if rust_analyzer || docs_rs {
        generate_fake_build_info();
    } else {
        build_info_build::build_script().collect_runtime_dependencies(depth);
    }
}

/// Generates fake build info so docs.rs works and rust analyzer speeds up
fn generate_fake_build_info() {
    let manifest_path: std::path::PathBuf = manifest_path();
    let manifest_contents: String = std::fs::read_to_string(manifest_path).unwrap();
    let manifest: toml::map::Map<String, toml::Value> =
        manifest_contents.parse::<toml::Table>().unwrap();
    let mut build_info_version: String = manifest
        .get("dependencies")
        .unwrap()
        .get("build-info")
        .unwrap()
        .get("version")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    build_info_version = build_info_version
        .chars()
        .filter(|&c| matches!(c, '.') | c.is_numeric())
        .collect();

    // Waiting for https://github.com/danielschemmel/build-info/pull/22
    let fake_data: String = format!("{{\"version\":\"{build_info_version}\",\"string\":\"KLUv/QCIfQUAYgkfGVDVAwMdwRLXXHpu1nWhFFma/2dL1xlougUumP6+APJ9j7KUcySnJLNNYnIltvVKqeC/kGIndHF1BHBIK4wv5CwLsGwLAIbYKL23nt62NWU9rV260vtN+lC7Gc6hQ88VJDnBTTvK2A2OlclP+nFC6Qv9pXpT45P+5vu7IxUg8C5MIG6uRGrJdMrMEWkifBPLCOMAwA1Yz4S7cwMRQhcZnAnHBXwkhgMFxxsKFg==\"}}");
    println!("cargo:rustc-env=BUILD_INFO={fake_data}");
}

/// Create C/C++/Python bindings
fn create_bindings() {
    let crate_directory: std::path::PathBuf = crate_dir();
    let package_name: String = std::env::var("CARGO_PKG_NAME").unwrap();

    create_binding("h", cbindgen::Language::C, &package_name, &crate_directory);
    create_binding(
        "hpp",
        cbindgen::Language::Cxx,
        &package_name,
        &crate_directory,
    );
    create_binding(
        "pxd",
        cbindgen::Language::Cython,
        &package_name.replace('-', "_"),
        &crate_directory,
    );
}

/// Create requested binding
fn create_binding(
    extension: &str,
    language: cbindgen::Language,
    package_name: &String,
    crate_directory: &std::path::PathBuf,
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
    if language == cbindgen::Language::Cython {
        header =
            "# cython: language_level=3\n\n".to_string() +
            "# This file exists to help facilitate modding this catgirl game engine...\n" +
            "# These generated bindings are either public domain or Unlicense where public domain does not exist";
    }

    let defines: std::collections::HashMap<String, String> = get_bindgen_defines();

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

    let mut config: cbindgen::Config = cbindgen::Config {
        namespace: Some(String::from("ffi")),
        header: Some(header),
        only_target_dependencies: true,
        no_includes: language == cbindgen::Language::Cython,
        parse: parse_config,
        language,
        defines,
        ..Default::default()
    };

    if language == cbindgen::Language::Cython {
        let crate_name: String = std::env::var("CARGO_PKG_NAME").unwrap();
        let header_filename: String = format!("\"<{crate_name}.h>\"");

        config.cython = cbindgen::CythonConfig {
            header: Some(header_filename),
            ..Default::default()
        };
    }

    cbindgen::generate_with_config(crate_directory, config)
        .unwrap()
        .write_to_file(output_file);
}

/// Define custom C defines macros
fn get_bindgen_defines() -> std::collections::HashMap<String, String> {
    let mut defines: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    // Features
    defines.insert(
        "feature = client".to_string(),
        "DEFINE_CLIENT_FEATURE".to_string(),
    );
    defines.insert(
        "feature = server".to_string(),
        "DEFINE_SERVER_FEATURE".to_string(),
    );
    defines.insert(
        "feature = embed-assets".to_string(),
        "DEFINE_EMBED_ASSETS_FEATURE".to_string(),
    );
    defines.insert(
        "feature = logging-subscriber".to_string(),
        "DEFINE_LOGGING_SUBSCRIBER_FEATURE".to_string(),
    );
    defines.insert(
        "feature = appimage".to_string(),
        "DEFINE_APPIMAGE_FEATURE".to_string(),
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
    defines.insert(
        "target_family = wasm".to_string(),
        "DEFINE_WASM_FAMILY".to_string(),
    );

    defines
}

/// Find the location of the crate's manifest
fn manifest_path() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_PATH").unwrap())
}

/// Find the location of the crate's directory
fn crate_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
}

/// Find the location of the `target/` directory. Note that this may be
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
/// variable
fn target_dir() -> std::path::PathBuf {
    if let Ok(target) = std::env::var("CARGO_TARGET_DIR") {
        std::path::PathBuf::from(target)
    } else {
        crate_dir().join("target")
    }
}

/// Print all environment variables
#[allow(dead_code)]
fn print_environment_vars() {
    let vars: std::env::Vars = std::env::vars();

    println!("cargo:warning=Environment Variables:");
    for (key, var) in vars {
        if is_likely_secret(&key) {
            println!("cargo:warning=Env: {key}: {}", mask_string(&var));
        } else {
            println!("cargo:warning=Env: {key}: {var}");
        }
    }
}

/// Determines if string represents a secret
fn is_likely_secret(key: &str) -> bool {
    match key.to_lowercase() {
        s if s.contains("password") => true,
        s if s.contains("secret") => true,
        s if s.contains("token") => true,
        s if s.contains("ssh") => true,
        s if s.contains("webhook") => true,
        s if s.contains("signing") => true,
        s if s.contains("api_key") => true,
        s if s.contains("release_key") => true,
        s if s.contains("release_store") => true,
        s if s.contains("account") => true,
        _ => false,
    }
}

/// Repeats a string an arbitrary number of times
fn repeat_string(repetitions: usize, value: &str) -> String {
    let mut buffer: Vec<&str> = Vec::new();

    for _ in 0..repetitions {
        buffer.push(value);
    }

    buffer.join("")
}

/// Masks a secret
fn mask_string(value: &str) -> String {
    let size: usize = value.chars().count();
    repeat_string(size, "*")
}
