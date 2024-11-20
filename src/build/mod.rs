use build_info::{chrono::Datelike, BuildInfo, CrateInfo};
use std::collections::BTreeMap;

use utils::println_string;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Module for grabbing and organizing dependencies
pub mod dependencies;

// Generate build_info() function at compile time
build_info::build_info!(
    /// Build info for crate
    pub fn build_info
);

/// Print the version of the engine
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn print_version() {
    let info: &BuildInfo = build_info();

    // The $... are proc macros - https://doc.rust-lang.org/reference/procedural-macros.html
    // Example: catgirl-engine v0.6.0 built with rustc 1.76.0 (07dca489a 2024-02-04) at 2024-02-20 07:40:40Z
    utils::println_string!(
        "{} v{} built with {} at {}",
        info.crate_info.name,
        info.crate_info.version,
        info.compiler,
        info.timestamp
    );
}

/// Print the dependencies of the engine
///
/// # Panics
///
/// May panic if the dependency license info cannot be unwrapped
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn print_dependencies() {
    let dependencies: BTreeMap<String, CrateInfo> = dependencies::get_all_dependencies();

    // Only add newline if there are dependencies to print
    #[cfg(not(target_family = "wasm"))]
    if !dependencies.is_empty() {
        println!();
    }

    // Print all dependencies
    // Loop through dependency list to print
    for (name, dep) in dependencies {
        let license: String = if dep.license.is_some() {
            dep.license.as_ref().unwrap().clone()
        } else {
            "Unknown".to_string()
        };

        println_string!("{} v{} - License {}", name, dep.version, license);
    }
}

/// Print the dependencies of the engine
///
/// # Panics
///
/// May panic if the license info cannot be unwrapped
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn print_license() {
    let info: &BuildInfo = build_info();

    // Example: Copyright (C) 2024 Alexis <@alexis@foxgirl.land> - Zlib License
    let year: i32 = info.timestamp.year();
    let author: String = if info.crate_info.authors.is_empty() {
        "Unknown".to_string()
    } else {
        info.crate_info.authors[0].clone()
    };

    let license: String = if info.crate_info.license.is_some() {
        info.crate_info.license.as_ref().unwrap().clone()
    } else {
        "Unknown".to_string()
    };

    utils::println_string!("Copyright (C) {} {} - {} License", year, author, license);
}

/// Prints extra build info
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn print_build_info() {
    let info: &BuildInfo = build_info();

    utils::println_string!(
        "Built for {} {} with {} profile",
        info.target.cpu.arch,
        info.target.os,
        info.profile
    );

    if let Some(git) = utils::build::get_version_control_build_info() {
        if git.dirty {
            utils::println_string!("Built from commit {}-dirty", git.commit_id);
        } else {
            utils::println_string!("Built from commit {}", git.commit_id);
        }
    }
}

/// Logs build info including version, commit, and compiled architecture
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn log_build_info() {
    // Logs debug information (useful for Android)
    let info: &BuildInfo = build_info();
    info!(
        "{} v{} built with {} at {}",
        info.crate_info.name, info.crate_info.version, info.compiler, info.timestamp
    );

    info!(
        "Built for {} {} with {} profile",
        info.target.cpu.arch, info.target.os, info.profile
    );

    if let Some(git) = utils::build::get_version_control_build_info() {
        if git.dirty {
            info!("Built from commit {}-dirty", git.commit_id);
        } else {
            info!("Built from commit {}", git.commit_id);
        }
    }
}
