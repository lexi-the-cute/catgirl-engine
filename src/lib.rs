#[macro_use]
extern crate log;

use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

mod game;
mod server;

// Run as Library
#[no_mangle]
pub extern "C" fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    game::setup_logger();
    debug!("Launched as library...");

    // This is to convert from C Main Args to Rust Main Args
    let _argc: isize = argc.try_into().unwrap();
    let _argv: *const *const u8 = argv as *const *const u8;

    return game::launch().try_into().unwrap();
}

#[no_mangle]
#[cfg(all(target_os = "android", feature = "client"))]
pub fn android_main(app: AndroidApp) {
    game::setup_logger();
    debug!("Launched as Android app...");

    let _: Result<(), String> = game::start_android(app);
}