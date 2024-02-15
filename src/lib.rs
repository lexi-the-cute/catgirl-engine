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

    return game::launch().try_into().unwrap();
}

#[no_mangle]
#[cfg(all(target_os = "android", feature = "client"))]
pub fn android_main(app: AndroidApp) {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    game::get_args();
    game::setup_logger();
    debug!("Launched as Android app...");

    let _: Result<(), String> = game::start_android(app);
}