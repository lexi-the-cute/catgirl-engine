use std::sync::OnceLock;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// To help check if [`exit()`] was already called
static EXITING: OnceLock<bool> = OnceLock::new();

/// Tells the game engine to start exiting next time it checks the exit status
pub fn set_exit() {
    if EXITING.get().is_some() {
        return;
    }

    let _ = EXITING.set(true);

    trace!("Engine is exiting...");
}

/// Retrieves if the game engine is exiting
pub fn is_exiting() -> bool {
    *EXITING.get().unwrap_or(&false)
}
