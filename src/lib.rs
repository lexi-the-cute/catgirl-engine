#[macro_use]
extern crate tracing;

mod game;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

// Run as Library
#[no_mangle]
pub extern "C" fn ce_start(_argc: c_int, _argv: *const *const c_char) -> c_int {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    // Rust obtains these args without me having to do anything special
    game::get_args();
    game::setup_logger();
    debug!("Launched as library...");

    match game::start() {
        Err(error) => {
            error!("{:?}", error);

            -1
        },
        _ => 0
    }
}

#[no_mangle]
#[cfg(all(target_os = "android", feature = "client"))]
pub fn android_main(app: AndroidApp) {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    game::get_args();
    game::setup_logger();
    debug!("Launched as Android app...");

    match game::start_android(app) {
        Err(error) => error!("{:?}", error),
        _ => ()
    }
}

#[no_mangle]
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn wasm_start() {
    // Temporary panic hook until logger is finished initializing
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    game::get_args();
    game::setup_logger();
    debug!("Launched as Wasm library...");

    match game::start() {
        Err(error) => error!("{:?}", error),
        _ => ()
    }
}