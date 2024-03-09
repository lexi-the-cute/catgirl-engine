/// Handles server side game loop
pub mod game_loop;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

/// Shortcut to [`game_loop::game_loop()`]
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
pub fn game_loop() -> Result<(), String> {
    game_loop::game_loop()
}

/// Shortcut to [`game_loop::game_loop()`] designed for Wasm
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn server_game_loop() -> Result<(), String> {
    game_loop()
}
