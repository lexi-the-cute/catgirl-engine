#![feature(start)]

#[macro_use] extern crate log;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

use wasm_bindgen::prelude::wasm_bindgen;

use core::ffi::{c_char, c_int};

mod game;
mod server;
mod client;

// Run as Library (e.g. Android and WebAssembly)
#[no_mangle]
#[wasm_bindgen]
pub extern fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    game::setup_logger();
    debug!("Launched as library...");
    
    // This is to convert from C Main Args to Rust Main Args
    let _argc: isize = argc.try_into().unwrap();
    let _argv: *const *const u8 = argv as *const *const u8;

    return game::launch(_argc, _argv).try_into().unwrap();
}

#[wasm_bindgen(start)]
fn start() -> Result<(), String> {
    game::setup_logger();
    debug!("Launched as WebAssembly library...");

    return game::start();
}

#[no_mangle]
#[cfg(all(target_os="android", feature="client"))]
pub fn android_main(app: AndroidApp) {
    game::setup_logger();
    debug!("Launched as Android app...");

    let _ = game::start_android(app);
}