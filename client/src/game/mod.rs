/// Handles client side game loop
pub mod game_loop;

#[cfg(target_os = "android")]
use std::sync::OnceLock;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
/// Holds a reference to the winit AndroidApp activity
pub static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

#[cfg(target_os = "android")]
/// Stores a reference to the winit AndroidApp activity
pub fn store_android_app(app: AndroidApp) {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);
}

/// Shortcut to [`game_loop::game_loop()`]
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
pub fn game_loop() -> Result<(), String> {
    game_loop::game_loop()
}
