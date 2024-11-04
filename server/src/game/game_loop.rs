use core::ffi::c_char;
use std::ffi::NulError;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Helper function to call [`server_game_loop()`] function from the C ABI
///
/// Returns empty C String if suceeded, else returns an error as a string
///
/// # Panics
///
/// May panic if the supplied string from an error contains a nul byte anywhere other than the end of the string
#[no_mangle]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn c_server_game_loop() -> *const c_char {
    if let Err(err) = server_game_loop() {
        let c_str_result: Result<*const c_char, NulError> = utils::get_c_string_from_rust(err);

        c_str_result.unwrap()
    } else {
        let c_str_result: Result<*const c_char, NulError> = utils::get_c_string_from_rust("");

        c_str_result.unwrap()
    }
}

/// Server side game loop
///
/// # Errors
///
/// Errors not implemented yet...
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
