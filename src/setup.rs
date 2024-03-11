use std::collections::BTreeMap;

use build_info::{chrono::Datelike, BuildInfo, CrateInfo};
use clap::Parser;
use client::game;
use utils::args::Args;
use wasm_bindgen::prelude::wasm_bindgen;

// Constants
#[cfg(target_os = "android")]
pub(crate) const TAG: &str = "CatgirlEngine";

// Generate build_info() function at compile time
build_info::build_info!(fn build_info);

/// Build info for crate
pub fn build_info_pub() -> &'static BuildInfo {
    build_info()
}

/// Process args for future use
#[wasm_bindgen]
pub fn process_args() {
    // Store assets path in separate variable
    game::store_assets_path(get_args().assets);

    trace!("Assets Path: {:?}", game::get_assets_path())
}

/// Retrieve parsed out command line arguments
pub fn get_args() -> Args {
    if utils::args::get_args().is_some() {
        utils::args::get_args().unwrap()
    } else {
        Args::parse()
    }
}

/// Get the list of dependencies used in the engine
pub(crate) fn get_dependencies(info: &BuildInfo) -> BTreeMap<String, CrateInfo> {
    let mut dependencies: BTreeMap<String, CrateInfo> = BTreeMap::new();
    let mut stack: Vec<&CrateInfo> = info.crate_info.dependencies.iter().collect();

    // Add each dependency only once
    while let Some(dep) = stack.pop() {
        if dep.name.starts_with("catgirl-engine") {
            // If one of my own crates, remove from results
            continue;
        }

        if dependencies
            .insert(dep.name.as_str().to_owned(), dep.to_owned())
            .is_none()
        {
            stack.extend(dep.dependencies.iter());
        }
    }

    dependencies
}

/// Get all dependencies from the workspace used to build the engine
pub fn get_all_dependencies() -> BTreeMap<String, CrateInfo> {
    let info: &BuildInfo = build_info();

    let mut dependencies: BTreeMap<String, CrateInfo> = get_dependencies(info);
    let mut util_dependencies: BTreeMap<String, CrateInfo> =
        get_dependencies(utils::setup::build_info_pub());

    dependencies.append(&mut util_dependencies);

    #[cfg(feature = "client")]
    {
        let mut client_dependencies: BTreeMap<String, CrateInfo> =
            get_dependencies(client::setup::build_info_pub());

        dependencies.append(&mut client_dependencies);
    }

    #[cfg(feature = "server")]
    {
        let mut server_dependencies: BTreeMap<String, CrateInfo> =
            get_dependencies(server::setup::build_info_pub());

        dependencies.append(&mut server_dependencies);
    }

    dependencies
}

/// Print the version of the engine
#[wasm_bindgen]
pub extern "C" fn print_version() {
    let info: &BuildInfo = build_info();

    if cfg!(target_family = "wasm") {
        // The $... are proc macros - https://doc.rust-lang.org/reference/procedural-macros.html
        // Example: catgirl-engine v0.6.0 built with rustc 1.76.0 (07dca489a 2024-02-04) at 2024-02-20 07:40:40Z
        debug!(
            "{} v{} built with {} at {}",
            info.crate_info.name, info.crate_info.version, info.compiler, info.timestamp
        );

        // Example: Copyright (C) 2024 Alexis <@alexis@foxgirl.land> - Zlib License
        debug!(
            "Copyright (C) {} {} - {} License",
            info.timestamp.year(),
            info.crate_info.authors[0],
            info.crate_info.license.as_ref().unwrap()
        );
    } else {
        // The $... are proc macros - https://doc.rust-lang.org/reference/procedural-macros.html
        // Example: catgirl-engine v0.6.0 built with rustc 1.76.0 (07dca489a 2024-02-04) at 2024-02-20 07:40:40Z
        println!(
            "{} v{} built with {} at {}",
            info.crate_info.name, info.crate_info.version, info.compiler, info.timestamp
        );

        // Example: Copyright (C) 2024 Alexis <@alexis@foxgirl.land> - Zlib License
        println!(
            "Copyright (C) {} {} - {} License",
            info.timestamp.year(),
            info.crate_info.authors[0],
            info.crate_info.license.as_ref().unwrap()
        );
    }
}

/// Print the dependencies of the engine
#[wasm_bindgen]
pub extern "C" fn print_dependencies() {
    let dependencies: BTreeMap<String, CrateInfo> = get_all_dependencies();

    // Only add newline if there are dependencies to print
    #[cfg(not(target_family = "wasm"))]
    if !dependencies.is_empty() {
        println!();
    }

    if cfg!(target_family = "wasm") {
        // Print all dependencies
        // Loop through dependency list to print
        for (name, dep) in dependencies {
            debug!(
                "{} v{} - License {}",
                name,
                dep.version,
                dep.license.as_ref().unwrap()
            )
        }
    } else {
        // Print all dependencies
        // Loop through dependency list to print
        for (name, dep) in dependencies {
            println!(
                "{} v{} - License {}",
                name,
                dep.version,
                dep.license.as_ref().unwrap()
            )
        }
    }
}

/// Setup the logger for the current platform
pub(crate) fn setup_logger() {
    if cfg!(target_os = "android") {
        // Limited Filter: trace,android_activity=debug,winit=debug
        // Stronger Filter: trace,android_activity=off,winit=off

        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(tracing::log::LevelFilter::Trace)
                .with_tag(TAG)
                .with_filter(
                    android_logger::FilterBuilder::new()
                        .parse("main=trace,catgirl_engine=trace")
                        .build(),
                ),
        );
    } else if cfg!(target_family = "wasm") {
        // https://github.com/daboross/fern/issues/134
        #[cfg(target_family = "wasm")]
        if let Err(_error) = fern::Dispatch::new()
            .level(tracing::log::LevelFilter::Off)
            .level_for("main", tracing::log::LevelFilter::Trace)
            .level_for("catgirl_engine", tracing::log::LevelFilter::Trace)
            .level_for("catgirl_engine_client", tracing::log::LevelFilter::Trace)
            .level_for("catgirl_engine_server", tracing::log::LevelFilter::Trace)
            .level_for("catgirl_engine_utils", tracing::log::LevelFilter::Trace)
            .chain(fern::Output::call(console_log::log))
            .apply()
        {
            warn!("Failed to initialize console logger...")
        }
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}

/// Setup the tracing subscriber to monitor the tracer
#[cfg(feature = "tracing-subscriber")]
pub(crate) fn setup_tracer() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    // Construct a subscriber to print formatted traces to stdout
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .without_time()
        .with_ansi(true)
        .init();
}

/// Setup a hook to catch panics for logging before shutdown
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(string) = info.payload().downcast_ref::<String>() {
            error!("Caught panic: {:?}", string);
        }
    }));
}

/// Determines if client or server and starts the engine
pub fn start() -> Result<(), String> {
    info!("Starting Game...");

    debug!("Setting panic hook...");
    set_panic_hook();

    // Allows handling properly shutting down with SIGINT
    #[cfg(any(target_family = "unix", target_family = "windows"))]
    {
        debug!("Setting SIGINT hook...");
        ctrlc::set_handler(move || {
            debug!("SIGINT (Ctrl+C) Was Called! Stopping...");
            std::process::exit(0);
        })
        .expect("Could not create Interrupt Handler (e.g. Ctrl+C)...");
    }

    debug!("Starting Main Loop...");

    #[cfg(feature = "server")]
    if cfg!(not(feature = "client")) || get_args().server {
        return server::game::game_loop();
    }

    #[cfg(feature = "client")]
    client::game::game_loop()
}
