#[macro_use]
extern crate tracing;

mod game;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

// Run as Library
#[no_mangle]
pub extern "C" fn ce_start(_argc: c_int, _argv: *const *const c_char) -> c_int {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    // Rust obtains these args without me having to do anything special
    // Print version and copyright info
    if game::get_args().version {
        game::print_version();
        return 0;
    }

    game::setup_logger();
    debug!("Launched as library...");

    match game::start() {
        Err(error) => {
            error!("{:?}", error);

            -1
        }
        _ => 0,
    }
}

#[no_mangle]
#[cfg(all(target_os = "android", feature = "client"))]
pub fn android_main(app: AndroidApp) {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    // Print version and copyright info
    if game::get_args().version {
        game::print_version();
        return ();
    }

    game::setup_logger();
    debug!("Launched as Android app...");

    client::game_loop::store_android_app(app);
    if let Err(error) = game::start() {
        error!("{:?}", error)
    }
}

#[no_mangle]
#[cfg(target_family = "wasm")]
#[cfg_attr(target_family = "wasm", wasm_bindgen(start))]
pub fn wasm_start() {
    // Temporary panic hook until logger is finished initializing
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    // Print version and copyright info
    if game::get_args().version {
        game::print_version();
        return ();
    }

    game::setup_logger();
    debug!("Launched as Wasm library...");

    if let Err(error) = game::start() {
        error!("{:?}", error)
    }
}
