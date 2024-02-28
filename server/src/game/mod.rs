pub mod game_loop;

pub fn game_loop() -> Result<(), String> {
    game_loop::game_loop()
}
