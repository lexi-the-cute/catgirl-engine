use std::ffi::{CString, NulError};

use core::ffi::c_char;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Repeats a string an arbitrary number of times
#[must_use]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn repeat_string(repetitions: usize, value: &str) -> String {
    let mut buffer: Vec<&str> = Vec::new();

    for _ in 0..repetitions {
        buffer.push(value);
    }

    buffer.join("")
}

/// Masks a secret
#[must_use]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn mask_string(value: String) -> String {
    let size: usize = value.chars().count();
    repeat_string(size, "*")
}

/// Determines if string represents a secret
#[must_use]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn is_likely_secret(key: String) -> bool {
    match key.to_lowercase() {
        // Very Likely
        s if s.contains("password") => true,
        s if s.contains("secret") => true,
        s if s.contains("token") => true,

        // Kinda Iffy
        s if s.contains("ssh") => true,
        s if s.contains("webhook") => true,
        s if s.contains("release_key") => true,
        s if s.contains("release_store") => true,

        // Iffy
        s if s.contains("account") => true,
        _ => false,
    }
}

/// Convert's Rust String to C String
///
/// # Errors
///
/// May return a `NulError` if the Rust string contained a nul byte anywhere other than the very end of the string
pub fn get_c_string_from_rust<S: AsRef<str>>(rstr: S) -> Result<*const c_char, NulError>
where
    Vec<u8>: From<S>,
{
    let cstr_result: Result<CString, NulError> = CString::new(rstr);
    if let Ok(cstr) = cstr_result {
        return Ok(cstr.as_ptr());
    }

    Err(cstr_result.unwrap_err())
}
