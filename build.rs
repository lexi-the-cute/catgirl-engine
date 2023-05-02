extern crate cbindgen;

use std::env;
use std::path::PathBuf;
use cbindgen::{Config, Language};

fn main() {
    let crate_directory: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name: String = env::var("CARGO_PKG_NAME").unwrap();

    create_binding("h", Language::C, &package_name, &crate_directory);
    create_binding("hpp", Language::Cxx, &package_name, &crate_directory);
    create_binding("pyx", Language::Cython, &package_name, &crate_directory);
}

fn create_binding(extension: &str, language: Language, package_name: &String, crate_directory: &String) {
    let output_file: String = target_dir()
    .join("binding")
    .join(format!("{}.{}", package_name, extension))
    .display()
    .to_string();

    let config: Config = Config {
        namespace: Some(String::from("ffi")),
        language: language,
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_directory, config)
        .unwrap()
        .write_to_file(&output_file);
}

/// Find the location of the `target/` directory. Note that this may be 
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR` 
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}