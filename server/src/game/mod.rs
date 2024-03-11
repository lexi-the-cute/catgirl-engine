/// Handles server side game loop
pub mod game_loop;

use wasm_bindgen::prelude::wasm_bindgen;

/// Shortcut to [`game_loop::game_loop()`]
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
pub fn game_loop() -> Result<(), String> {
    game_loop::game_loop()
}

/// Shortcut to [`game_loop::game_loop()`] designed for Wasm
///
/// [`game_loop::game_loop()`]: crate::game::game_loop::game_loop()
#[wasm_bindgen]
pub fn server_game_loop() -> Result<(), String> {
    game_loop()
}
