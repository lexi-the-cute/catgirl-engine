/// Handles client side game loop
pub mod game_loop;

use std::{path::PathBuf, sync::OnceLock};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
/// Holds a reference to the winit AndroidApp activity
pub static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

/// Holds a path to the assets directory
static ASSETS_PATH: OnceLock<PathBuf> = OnceLock::new();

#[cfg(target_os = "android")]
/// Stores a reference to the winit AndroidApp activity
pub fn store_android_app(app: AndroidApp) {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);
}

#[cfg(target_os = "android")]
/// Retrieves a reference to the stored winit AndroidApp activity
pub fn get_android_app() -> AndroidApp {
    ANDROID_APP
        .get()
        .expect("Could not get stored reference to AndroidApp")
        .clone()
}

/// Stores a custom path to the assets directory
pub fn store_assets_path(path: PathBuf) {
    let _path: &PathBuf = ASSETS_PATH.get_or_init(|| path);
}

/// Get assets path
///
/// # Panics
///
/// The path to the assets directory may be invalid
pub fn get_assets_path() -> PathBuf {
    if ASSETS_PATH.get().is_some() {
        ASSETS_PATH.get().unwrap().clone()
    } else {
        // If process_args is ran, this should never be called
        PathBuf::from("assets")
    }
}

/// Shortcut to [`game_loop::client_game_loop()`]
///
/// [`game_loop::client_game_loop()`]: crate::game::game_loop::client_game_loop()
///
/// # Errors
///
/// The event loop may not be created
///
/// # Panics
///
/// The event loop may not be created
pub fn game_loop() -> Result<(), String> {
    game_loop::client_game_loop()
}
