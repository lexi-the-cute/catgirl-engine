/// Server side game loop
///
/// # Errors
///
/// Errors not implemented yet...
pub fn server_game_loop() -> Result<(), String> {
    debug!("Started server game loop...");
    error!("Dedicated server game loop not implemented yet...");

    loop {
        if utils::exit::is_exiting() {
            break;
        }
    }

    Ok(())
}
