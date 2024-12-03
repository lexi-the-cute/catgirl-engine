/// Module for grabbing and organizing dependencies
mod dependencies;

use build_info::{chrono::Datelike, BuildInfo, CrateInfo};
use std::collections::BTreeMap;

use utils::println_string;

// Generate build_info() function at compile time
build_info::build_info!(
    /// Build info for crate
    fn build_info
);

// Generates the macros_build_info() function at compile time
//
// Procedural Macro crates currently cannot export anything other than procedural macros
// macros::macros_build_info!();

/// Print the version of the engine
pub(super) fn print_version() {
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
pub(super) fn print_dependencies() {
    let dependencies: BTreeMap<String, CrateInfo> = dependencies::get_all_dependencies();

    // Only add newline if there are dependencies to print
    #[cfg(not(target_family = "wasm"))]
    if !dependencies.is_empty() {
        println!();
    }

    // Print all dependencies
    // Loop through dependency list to print
    for (name, dep) in dependencies {
        let license: &String = if dep.license.is_some() {
            dep.license.as_ref().unwrap()
        } else {
            &"Unknown".to_string()
        };

        println_string!("{} v{} - License {}", name, dep.version, license);
    }
}

/// Print the dependencies of the engine
///
/// # Panics
///
/// May panic if the license info cannot be unwrapped
pub(super) fn print_license() {
    let info: &BuildInfo = build_info();

    // Example: Copyright (C) 2024 Alexis <@alexis@fearness.org> - Zlib License
    let year: i32 = info.timestamp.year();
    let author: &String = if info.crate_info.authors.is_empty() {
        &"Unknown".to_string()
    } else {
        &info.crate_info.authors[0]
    };

    let license: &String = if info.crate_info.license.is_none() {
        &"Unknown".to_string()
    } else {
        info.crate_info.license.as_ref().unwrap()
    };

    utils::println_string!("Copyright (C) {} {} - {} License", year, author, license);
}

/// Prints extra build info
pub(super) fn print_build_info() {
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
pub(super) fn log_build_info() {
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

/// Helps those who fork the project comply with the license
pub(super) fn license_compliance_helper() {
    let info: &BuildInfo = build_info();
    let crate_name: &String = &info.crate_info.name;
    let repo_url: String = "https://github.com/foxgirl-labs/catgirl-engine".to_string();

    if !crate_name.starts_with("catgirl-engine") {
        info!("{crate_name} is based off of the Catgirl Engine from {repo_url}");
    }
}
