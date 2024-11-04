use core::ffi::c_char;
use std::ffi::CString;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Helper function to call [`server_game_loop()`] function from the C ABI
///
/// Returns empty C String if suceeded, else returns an error as a string
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn c_server_game_loop() -> *const c_char {
    match server_game_loop() {
        Err(err) => {
            let c_str = CString::new(err).unwrap();

            c_str.as_ptr()
        }
        _ => {
            let c_str = CString::new("").unwrap();

            c_str.as_ptr()
        }
    }
}

/// Server side game loop
///
/// # Errors
///
/// Errors not implemented yet...
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn server_game_loop() -> Result<(), String> {
    debug!("Started server game loop...");
    error!("Dedicated server game loop not implemented yet...");

    loop {
        if utils::setup::is_exiting() {
            break;
        }
    }

    Ok(())
}
