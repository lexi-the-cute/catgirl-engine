#![feature(start)]

#[macro_use]
extern crate log;

#[cfg(not(target_os = "android"))]
use core::ffi::{c_char, c_int};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(any(all(target_arch = "wasm32", target_os="unknown"), all(target_arch = "wasm64", target_os="unknown")))]
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

mod client;
mod game;
mod server;

// Run as Library
#[no_mangle]
#[cfg(not(target_os = "android"))]
pub extern "C" fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    game::setup_logger();
    debug!("Launched as library...");

    // This is to convert from C Main Args to Rust Main Args
    let _argc: isize = argc.try_into().unwrap();
    let _argv: *const *const u8 = argv as *const *const u8;

    return game::launch(_argc, _argv).try_into().unwrap();
}

// TODO: Get working in browser and possibly wasmtime (https://wasmtime.dev/)
#[wasm_bindgen(start)]
#[cfg(any(all(target_arch = "wasm32", target_os="unknown"), all(target_arch = "wasm64", target_os="unknown")))]
fn wasm_start() -> Result<(), JsError> {
    game::setup_logger();
    debug!("Launched as WebAssembly library...");

    let _: Result<(), String> = game::start();

    Ok(())
}

#[no_mangle]
#[cfg(all(target_os = "android", feature = "client"))]
pub fn android_main(app: AndroidApp) {
    game::setup_logger();
    debug!("Launched as Android app...");

    let _: Result<(), String> = game::start_android(app);
}
