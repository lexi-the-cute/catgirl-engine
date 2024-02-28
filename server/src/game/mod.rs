/// Handles server side game loop
pub mod game_loop;

/// Shortcut to game_loop::game_loop()
pub fn game_loop() -> Result<(), String> {
    game_loop::game_loop()
}
