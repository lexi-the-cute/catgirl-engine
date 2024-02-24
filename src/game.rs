use std::collections::BTreeMap;

use build_info::{chrono::Datelike, BuildInfo, CrateInfo};
use clap::Parser;

// Constants
#[cfg(target_os = "android")]
pub const TAG: &str = "CatgirlEngine";

// Generate build_info() function at compile time
build_info::build_info!(fn build_info);

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub(crate) struct Args {
    /// Start the engine in dedicated server mode
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// Display version and copyright info
    #[arg(short, long, default_value_t = false)]
    pub(crate) version: bool,
}

#[no_mangle]
pub fn get_args() -> Args {
    Args::parse()
}

pub(crate) fn get_dependencies(info: &BuildInfo) -> BTreeMap<&str, &CrateInfo> {
    let mut dependencies: BTreeMap<&str, &CrateInfo> = BTreeMap::new();
    let mut stack: Vec<&CrateInfo> = info.crate_info.dependencies.iter().collect();

    // Add each dependency only once
    while let Some(dep) = stack.pop() {
        if dep.name.starts_with("catgirl-engine") {
            // If one of my own crates, remove from results
            continue;
        }

        if dependencies.insert(dep.name.as_str(), dep).is_none() {
            stack.extend(&dep.dependencies);
        }
    }

    dependencies
}

pub(crate) fn print_version() {
    let info: &BuildInfo = build_info();

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

    let dependencies: BTreeMap<&str, &CrateInfo> = get_dependencies(info);

    // Only add newline if there are dependencies to print
    if !dependencies.is_empty() {
        println!();
    }

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
                        .parse("trace,android_activity=off,winit=off")
                        .build(),
                ),
        );
    } else if cfg!(target_family = "wasm") {
        #[cfg(target_family = "wasm")]
        if let Err(_error) = console_log::init_with_level(tracing::log::Level::Debug) {
            warn!("Failed to initialize console logger...")
        }
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}

#[cfg(feature = "tracing-subscriber")]
pub(crate) fn setup_tracer() {
    // Construct a subscriber to print formatted traces to stdout
    let subscriber: tracing_subscriber::FmtSubscriber =
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(false)
            .finish();

    // Process future traces
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        warn!(
            "Failed to set the tracing subscriber as the global default... Message: {:?}",
            error
        )
    }
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(string) = info.payload().downcast_ref::<String>() {
            error!("Caught panic: {:?}", string);
        }
    }));
}

pub fn start() -> Result<(), String> {
    // let (tx, rx) = mpsc::channel();
    info!("Starting Game...");

    debug!("Setting panic hook...");
    set_panic_hook();

    // Allows handling properly shutting down with SIGINT
    #[cfg(not(target_family = "wasm"))]
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
        return server::game_loop::game_loop();
    }

    #[cfg(feature = "client")]
    client::game_loop::game_loop()
}
