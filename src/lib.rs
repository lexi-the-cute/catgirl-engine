#[macro_use]
extern crate tracing;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

mod game;
mod common;
mod client;

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