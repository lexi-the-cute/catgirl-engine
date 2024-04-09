use std::sync::OnceLock;

/// To help check if [`exit()`] was already called
static EXITING: OnceLock<bool> = OnceLock::new();

// Generate build_info() function at compile time
build_info::build_info!(
    /// Build info for crate
    pub fn build_info
);

/// Exits the game engine
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn exit() {
    if EXITING.get().is_some() {
        return;
    }

    let _ = EXITING.set(true);

    trace!("Engine is exiting...");
    std::process::exit(0);
}
