#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Server side game loop
///
/// # Errors
///
/// Errors not implemented yet...
// TODO (BIND): Implement `extern "C"`
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "Rust" fn server_game_loop() -> Result<(), String> {
    debug!("Started server game loop...");
    // loop {
    // TODO: Implement
    // }

    Ok(())
}
