//! Starting point for the game engine as a library

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
mod setup;

/// Module for storing and using build data
mod build;

/// Module for storing resources
mod resources;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

// Run as Library
/// Catgirl Engine start
///
/// The starting point when calling as a generic library
pub fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    #[cfg(feature = "embed-resources")]
    utils::resources::store_embedded_resources(resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

    // Create a vector of args from C styled args
    // We create a new pointer so we guarantee the pointer we are passing is valid
    // This doesn't say anything about the underlying data, but that's the responsibility of
    //   parse_args_from_c(...) to validate
    let args: Option<Vec<String>>;
    unsafe {
        args = utils::args::parse_args_from_c(argc, argv.cast::<*const *const c_char>());
    }

    // Override Clap's args
    if let Some(args) = args {
        utils::args::set_parsed_args(args);
    }

    // Print version and copyright info
    if setup::get_args().version {
        build::print_version();
        build::print_build_info();
        build::print_dependencies();

        println!();
        build::print_license();
        return 0;
    }

    // Process args for future use
    setup::process_args();

    debug!("Launched as library...");
    build::log_build_info();

    match setup::start() {
        Err(error) => {
            error!("{:?}", error);

            1
        }
        _ => 0,
    }
}

#[cfg(all(target_os = "android", feature = "client"))]
/// The starting point when loaded as an Android app
fn android_main(app: AndroidApp) {
    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    #[cfg(feature = "embed-resources")]
    utils::resources::store_embedded_resources(resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

    // Print version and copyright info
    if setup::get_args().version {
        build::print_version();
        build::print_build_info();
        build::print_dependencies();
        build::print_license();
        return ();
    }

    // Process args for future use
    setup::process_args();

    debug!("Launched as Android app...");
    build::log_build_info();

    client::game::store_android_app(app);
    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
/// The starting point when loaded via wasm bindgen
fn wasm_start() {
    // Temporary panic hook until logger is finished initializing
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    #[cfg(feature = "embed-resources")]
    utils::resources::store_embedded_resources(resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

    // Print version and copyright info
    if setup::get_args().version {
        build::print_version();
        build::print_build_info();
        build::print_dependencies();
        build::print_license();
        return ();
    }

    // Process args for future use
    setup::process_args();

    debug!("Launched as Wasm library...");
    build::log_build_info();

    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}
