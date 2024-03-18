/// Handles client side game loop
pub mod game_loop;

use std::{path::PathBuf, sync::OnceLock};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
/// Holds a reference to the winit AndroidApp activity
pub static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

/// Holds a path to the assets directory
static ASSETS_PATH: OnceLock<PathBuf> = OnceLock::new();

#[cfg(target_os = "android")]
/// Stores a reference to the winit AndroidApp activity
// TODO (BIND): Implement `extern "C"`
pub fn store_android_app(app: AndroidApp) {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);
}

/// Stores a custom path to the assets directory
// TODO (BIND): Implement `#[wasm_bindgen]` and `extern "C"`
pub fn store_assets_path(path: PathBuf) {
    let _path: &PathBuf = ASSETS_PATH.get_or_init(|| path);
}

/// Get assets path
// TODO (BIND): Implement `#[wasm_bindgen]` and `extern "C"`
pub fn get_assets_path() -> PathBuf {
    if ASSETS_PATH.get().is_some() {
        ASSETS_PATH.get().unwrap().clone()
    } else {
        // If process_args is ran, this should never be called
        PathBuf::from("assets")
    }
}

/// Shortcut to [`game_loop::game_loop()`]
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
// TODO (BIND): Implement `#[wasm_bindgen]` and `extern "C"`
pub fn game_loop() -> Result<(), String> {
    game_loop::game_loop()
}

/// Shortcut to [`game_loop::game_loop()`] designed for Wasm
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
// TODO (BIND): Implement `extern "C"`
#[wasm_bindgen]
pub fn client_game_loop() -> Result<(), String> {
    game_loop()
}
