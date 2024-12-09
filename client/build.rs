//! Build script for crate

/// Main function
fn main() {
    // Debug environment
    // print_environment_vars();

    // Generate build info
    generate_build_info();
}

/// Generate build info
fn generate_build_info() {
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

/// Find the location of the crate's manifest
fn manifest_path() -> std::path::PathBuf {
    std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_PATH").unwrap())
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
        // Very Likely
        s if s.contains("password") => true,
        s if s.contains("secret") => true,
        s if s.contains("token") => true,

        // Kinda Iffy
        s if s.contains("ssh") => true,
        s if s.contains("webhook") => true,
        s if s.contains("release_key") => true,
        s if s.contains("release_store") => true,

        // Iffy
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
