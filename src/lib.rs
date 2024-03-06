//! Starting point for the catgirl-engine as a library

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
pub mod setup;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

// Run as Library
/// Catgirl Engine start
///
/// The starting point when calling as a generic library
#[no_mangle]
pub extern "C" fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Override Clap's Args
    parse_args_from_c(argc, argv);

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        return 0;
    }

    setup::setup_logger();
    debug!("Launched as library...");

    match setup::start() {
        Err(error) => {
            error!("{:?}", error);

            -1
        }
        _ => 0,
    }
}

/// Parse arguments from C and send to the Clap library
fn parse_args_from_c(argc: c_int, argv: *const *const c_char) {
    use clap::Parser;
    use core::ffi::CStr;

    // Parse array out of argv
    let c_args: &[*const c_char] = unsafe { std::slice::from_raw_parts(argv, argc as usize) };

    let mut args: Vec<String> = vec![];
    for &arg in c_args {
        let c_str: &CStr = unsafe { CStr::from_ptr(arg) };
        eprintln!("C: {:?}", c_str.to_str());
        let str_slice: &str = c_str.to_str().unwrap();

        args.push(str_slice.to_owned());
    }

    utils::args::Args::parse_from(args.iter());
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
        return ();
    }

    setup::setup_logger();
    debug!("Launched as Android app...");

    client::game::store_android_app(app);
    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}

#[no_mangle]
#[cfg(target_family = "wasm")]
#[cfg_attr(target_family = "wasm", wasm_bindgen(start))]
/// The starting point when loaded via wasm bindgen
pub fn wasm_start() {
    // Temporary panic hook until logger is finished initializing
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        return ();
    }

    setup::setup_logger();
    debug!("Launched as Wasm library...");

    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}
