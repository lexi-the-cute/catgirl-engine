use core::ffi::c_char;
use core::{any::Any, marker::Send};

use std::collections::BTreeMap;
use std::ffi::NulError;

use build_info::{chrono::Datelike, BuildInfo, CrateInfo};
use clap::Parser;
use utils::{args::Args, println_string};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

// Constants
#[cfg(target_os = "android")]
pub(crate) const TAG: &str = "CatgirlEngine";

// Generate build_info() function at compile time
build_info::build_info!(
    /// Build info for crate
    pub fn build_info
);

/// Process args for future use
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn process_args() {
    // Store assets path in separate variable
    #[cfg(feature = "client")]
    client::game::store_assets_path(get_args().assets);

    // Uninstall desktop files
    #[cfg(all(feature = "client", target_os = "linux", not(feature = "no_lint")))]
    if get_args().uninstall_desktop_files {
        trace!("Uninstalling desktop files...");
        let _ = client::uninstall_desktop_files();
    }

    // Install desktop files
    #[cfg(all(feature = "client", target_os = "linux", not(feature = "no_lint")))]
    if get_args().install_desktop_files {
        trace!("Installing desktop files...");
        let _ = client::install_desktop_files();
    }

    if get_args().print_environment_variables {
        trace!("Printing environment variables...");
        utils::print_environment_vars();
    }

    #[cfg(feature = "client")]
    trace!("Assets Path: {:?}", client::game::get_assets_path());
}

/// Retrieve parsed out command line arguments
///
/// # Panics
///
/// This may panic if the args cannot be unwrapped
#[must_use]
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
            .insert(dep.name.as_str().to_string(), dep.to_owned())
            .is_none()
        {
            stack.extend(dep.dependencies.iter());
        }
    }

    dependencies
}

/// Get all dependencies from the workspace used to build the engine
#[must_use]
pub fn get_all_dependencies() -> BTreeMap<String, CrateInfo> {
    let info: &BuildInfo = build_info();

    let mut dependencies: BTreeMap<String, CrateInfo> = get_dependencies(info);
    let mut util_dependencies: BTreeMap<String, CrateInfo> =
        get_dependencies(utils::setup::build_info());

    dependencies.append(&mut util_dependencies);

    #[cfg(feature = "client")]
    {
        let mut client_dependencies: BTreeMap<String, CrateInfo> =
            get_dependencies(client::setup::build_info());

        dependencies.append(&mut client_dependencies);
    }

    #[cfg(feature = "server")]
    {
        let mut server_dependencies: BTreeMap<String, CrateInfo> =
            get_dependencies(server::setup::build_info());

        dependencies.append(&mut server_dependencies);
    }

    dependencies
}

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
    let dependencies: BTreeMap<String, CrateInfo> = get_all_dependencies();

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

    if let Some(git) = utils::get_version_control_build_info() {
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

    if let Some(git) = utils::get_version_control_build_info() {
        if git.dirty {
            info!("Built from commit {}-dirty", git.commit_id);
        } else {
            info!("Built from commit {}", git.commit_id);
        }
    }
}

/// Setup the logger for the current platform
#[cfg(feature = "logging-subscriber")]
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
        let mut builder = pretty_env_logger::formatted_builder();
        if let Ok(s) = ::std::env::var("RUST_LOG") {
            // Set logger according to RUST_LOG environment variable
            builder.parse_filters(&s);
        } else {
            // Failed to find RUST_LOG environment variable
            builder
                .default_format()
                .filter(Some("main"), tracing::log::LevelFilter::Info)
                .filter(Some("catgirl_engine"), tracing::log::LevelFilter::Info)
                .filter(
                    Some("catgirl_engine_client"),
                    tracing::log::LevelFilter::Info,
                )
                .filter(
                    Some("catgirl_engine_server"),
                    tracing::log::LevelFilter::Info,
                )
                .filter(
                    Some("catgirl_engine_utils"),
                    tracing::log::LevelFilter::Info,
                );
        }

        builder.try_init().unwrap();
    }
}

/// Setup the tracing subscriber to monitor the tracer
#[cfg(feature = "tracing-subscriber")]
pub(crate) fn setup_tracer() {
    if cfg!(target_family = "wasm") {
        error!("The tracing-subscriber feature is currently not supported on wasm...");
        return;
    }

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

/// Get the type of the variable
fn get_type_of<T: ?Sized>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

/// TODO: Fix output of the location info
/// Setup a hook to catch panics for logging before shutdown
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let location_string = if let Some(location) = info.location() {
            format!(
                "in file {} at line:column {}:{}",
                location.file(),
                location.line(),
                location.column()
            )
        } else {
            "".to_string()
        };

        if let Some(string) = info.payload().downcast_ref::<String>() {
            error!("Caught panic{location_string}: {string}");
        } else {
            let payload: &(dyn Any + Send) = info.payload();
            let payload_type: String = get_type_of(payload);

            error!(
                "Caught panic{location_string} with type {payload_type}: {:?}",
                payload
            );
        }

        utils::setup::set_exit();
    }));
}

/// This functions intentionally triggers a panic
///
/// # Panics
///
/// Always
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn trigger_panic() {
    panic!("Intentionally triggered a panic for debugging...");
}

/// Helper function to call [`start()`] function from the C ABI
///
/// Returns empty C String if suceeded, else returns an error as a string
///
/// # Panics
///
/// May panic if the supplied string from an error contains a nul byte anywhere other than the end of the string
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn c_start() -> *const c_char {
    if let Err(err) = start() {
        let c_str_result: Result<*const c_char, NulError> = utils::get_c_string_from_rust(err);

        c_str_result.unwrap()
    } else {
        let c_str_result: Result<*const c_char, NulError> = utils::get_c_string_from_rust("");

        c_str_result.unwrap()
    }
}

/// Determines if client or server and starts the engine
///
/// # Panics
///
/// This may fail to set the ctrl+c handler
///
/// # Errors
///
/// This may fail to set the ctrl+c handler
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
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
            utils::setup::set_exit();

            #[cfg(feature = "client")]
            if !get_args().server {
                #[cfg(not(target_family = "wasm"))]
                client::game::game_loop::advance_event_loop();
            }
        })
        .expect("Could not create Interrupt Handler (e.g. Ctrl+C)...");
    }

    debug!("Starting main loop...");

    #[cfg(feature = "server")]
    if cfg!(not(feature = "client")) || get_args().server {
        // Server exists, client may exist
        return server::game::game_loop();
    }

    // Client exists, server may exist
    #[cfg(feature = "client")]
    return client::game::game_loop();

    // Server doesn't exist, client doesn't exist
    #[cfg(not(feature = "client"))]
    Err("Neither the client nor server features were configured at build time...".to_string())
}
