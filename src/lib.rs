//! Starting point for the catgirl-engine as a library

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
pub mod setup;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

use wasm_bindgen::prelude::wasm_bindgen;

extern crate wasm_bindgen;

// Run as Library
/// Catgirl Engine start
///
/// The starting point when calling as a generic library
#[no_mangle]
#[wasm_bindgen]
pub extern "C" fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

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
        setup::print_version();
        setup::print_dependencies();
        return 0;
    }

    // Setup logger for debugging
    setup::setup_logger();
    debug!("Launched as library...");

    // Process args for future use
    setup::process_args();

    match setup::start() {
        Err(error) => {
            error!("{:?}", error);

            -1
        }
        _ => 0,
    }
}

#[no_mangle]
#[cfg(all(target_os = "android", feature = "client"))]
/// The starting point when loaded as an Android app
pub fn android_main(app: AndroidApp) {
    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        setup::print_dependencies();
        return ();
    }

    // Setup logger for debugging
    setup::setup_logger();
    debug!("Launched as Android app...");

    // Process args for future use
    setup::process_args();

    client::game::store_android_app(app);
    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
/// The starting point when loaded via wasm bindgen
pub fn wasm_start() {
    // Temporary panic hook until logger is finished initializing
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        setup::print_dependencies();
        return ();
    }

    // Setup logger for debugging
    setup::setup_logger();
    debug!("Launched as Wasm library...");

    // Process args for future use
    setup::process_args();

    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}
