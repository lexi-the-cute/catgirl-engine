//! Utilities for the catgirl-engine crate

#![warn(missing_docs)]

use core::ffi::c_char;
use std::env;
use std::ffi::{CString, NulError, OsStr};

use build_info::GitInfo;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

/// Module for command line arguments
pub mod args;

/// Handles setup
pub mod setup;

/// Prints strings in a format usable by the target platform
///
/// Will log with info for Wasm while printing a line on other platforms
#[macro_export]
macro_rules! println_string {
    // Single Arg
    ($arg:tt) => {{
        if cfg!(target_family = "wasm") {
            info!("{}", $arg);
        } else {
            println!("{}", $arg);
        }
    }};

    // Multiple Args
    ($fmt:expr, $($arg:tt)+) => {{
        if cfg!(target_family = "wasm") {
            info!($fmt, $($arg)*);
        } else {
            println!($fmt, $($arg)*);
        }
    }};
}

/// Checks if string matches environment variable
///
/// # Panics
///
/// May panic if environment var cannot be unwrapped
#[must_use]
pub fn matches_environment_var<S: AsRef<OsStr>>(key: S, value: S) -> bool
where
    std::string::String: PartialEq<S>,
{
    let environment_var: Result<String, env::VarError> = env::var(key);
    environment_var.is_ok() && environment_var.unwrap() == value
}

/// Get value of the requested environment variable
///
/// # Panics
///
/// May panic if environment var cannot be unwrapped
#[must_use]
pub fn get_environment_var<S: AsRef<OsStr>>(key: S) -> Option<String> {
    let environment_var: Result<String, env::VarError> = env::var(key);

    if let Ok(environment_var) = environment_var {
        Some(environment_var)
    } else {
        None
    }
}

/// Print all environment variables
#[no_mangle]
pub extern "C" fn print_environment_vars() {
    let vars: std::env::Vars = std::env::vars();

    println_string!("Environment Variables:");
    for (key, var) in vars {
        if is_likely_secret(key.clone()) {
            println_string!("{}: {}", key, mask_secret(var));
        } else {
            println_string!("{}: {}", key, var);
        }
    }
}

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
pub fn mask_secret(value: String) -> String {
    let size: usize = value.chars().count();
    repeat_string(size, "*")
}

/// Determines if string represents a secret
#[must_use]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn is_likely_secret(key: String) -> bool {
    match key.to_lowercase() {
        s if s.contains("password") => true,
        s if s.contains("secret") => true,
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

/// Retrieves the commit hash of the repo when this was built
#[must_use]
pub fn get_version_control_build_info() -> Option<GitInfo> {
    crate::setup::build_info()
        .version_control
        .as_ref()?
        .git()
        .cloned()
}
