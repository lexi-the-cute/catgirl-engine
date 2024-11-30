/// Handles client side game loop
mod game_loop;

use std::{path::PathBuf, sync::OnceLock};

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

pub use game_loop::advance_event_loop;
pub use game_loop::client_game_loop as game_loop;

#[cfg(target_os = "android")]
/// Holds a reference to the winit AndroidApp activity
static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

/// Holds a path to the resources directory
static RESOURCES_PATH: OnceLock<PathBuf> = OnceLock::new();

#[cfg(target_os = "android")]
/// Stores a reference to the winit AndroidApp activity
pub fn store_android_app(app: AndroidApp) {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);
}

// #[cfg(target_os = "android")]
// /// Retrieves a reference to the stored winit AndroidApp activity
// fn get_android_app() -> AndroidApp {
//     ANDROID_APP
//         .get()
//         .expect("Could not get stored reference to AndroidApp")
//         .clone()
// }

/// Stores a custom path to the resources directory
pub fn store_resources_path(path: PathBuf) {
    let _path: &PathBuf = RESOURCES_PATH.get_or_init(|| path);
}

/// Get resources path
///
/// # Panics
///
/// The path to the resources directory may be invalid
pub fn get_resources_path() -> PathBuf {
    if RESOURCES_PATH.get().is_some() {
        RESOURCES_PATH.get().unwrap().clone()
    } else {
        // If process_args is ran, this should never be called
        PathBuf::from("resources")
    }
}
