/// Handles server side game loop
pub mod game_loop;

/// Shortcut to [`game_loop::server_game_loop()`]
///
/// [`game_loop::server_game_loop()`]: crate::game::game_loop::server_game_loop()
///
/// # Errors
///
/// Errors not implemented yet...
// TODO (BIND): Implement `extern "C"`
pub fn game_loop() -> Result<(), String> {
    game_loop::server_game_loop()
}
