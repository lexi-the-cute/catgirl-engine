extern crate cbindgen;

use cbindgen::{Config, Language};
use std::collections::HashMap;
use std::env::{self, Vars};
use std::path::PathBuf;

fn main() {
    // Bindings are only usable when building libs
    create_bindings();
}

fn create_bindings() {
    let crate_directory: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name: String = env::var("CARGO_PKG_NAME").unwrap();

    create_binding("h", Language::C, &package_name, &crate_directory);
    create_binding("hpp", Language::Cxx, &package_name, &crate_directory);
    create_binding(
        "pxd",
        Language::Cython,
        &package_name.replace("-", "_"),
        &crate_directory,
    );
}

fn create_binding(
    extension: &str,
    language: Language,
    package_name: &String,
    crate_directory: &String,
) {
    let output_file: String = target_dir()
        .join("binding")
        .join(format!("{}.{}", package_name, extension))
        .display()
        .to_string();

    let mut header: String = "".to_owned() +
        "/*\n" +
        " * This file exists to help facilitate modding this catgirl game engine...\n" + 
        " * These generated bindings are either public domain or Unlicense where public domain does not exist\n" + 
        " */";
    if language == Language::Cython {
        header =
            "# cython: language_level=3\n\n".to_owned() +
            "# This file exists to help facilitate modding this catgirl game engine...\n" +
            "# These generated bindings are either public domain or Unlicense where public domain does not exist";
    }

    let defines: HashMap<String, String> = get_bindgen_defines();

    let mut config: Config = cbindgen::Config::default();
    config.namespace = Some(String::from("ffi"));
    config.header = Some(String::from(header));
    config.language = language;
    config.only_target_dependencies = true;
    config.no_includes = language == Language::Cython;
    config.defines = defines;

    cbindgen::generate_with_config(&crate_directory, config)
        .unwrap()
        .write_to_file(&output_file);
}

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

#[allow(dead_code)]
fn print_environment_vars() {
    let vars: Vars = std::env::vars();

    for (key, var) in vars {
        println!("cargo:warning=EV: {key}: {var}");
    }
}