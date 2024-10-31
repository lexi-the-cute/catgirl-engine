use std::sync::OnceLock;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// To help check if [`exit()`] was already called
static EXITING: OnceLock<bool> = OnceLock::new();

// Generate build_info() function at compile time
build_info::build_info!(
    /// Build info for crate
    pub fn build_info
);

/// Tells the game engine to start exiting next time it checks the exit status
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn set_exit() {
    if EXITING.get().is_some() {
        return;
    }

    let _ = EXITING.set(true);

    trace!("Engine is exiting...");
}

/// Retrieves if the game engine is exiting
pub extern "C" fn is_exiting() -> bool {
    EXITING.get().unwrap_or(&false).clone()
}
