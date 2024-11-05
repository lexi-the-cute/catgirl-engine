//! Utilities for the catgirl-engine crate

#![warn(missing_docs)]

use core::ffi::c_char;
use std::env;
use std::ffi::{CString, NulError};

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

/// Module for command line arguments
pub mod args;

/// Handles setup
pub mod setup;

/// Checks if string matches environment variable
///
/// # Panics
///
/// May panic if environment var cannot be unwrapped
#[must_use]
pub fn matches_environment_var(key: &str, value: &str) -> bool {
    let environment_var: Result<String, env::VarError> = env::var(key);
    environment_var.is_ok() && environment_var.unwrap() == value
}

/// Get value of the requested environment variable
///
/// # Panics
///
/// May panic if environment var cannot be unwrapped
#[must_use]
pub fn get_environment_var(key: &str) -> Option<String> {
    let environment_var: Result<String, env::VarError> = env::var(key);

    if let Ok(environment_var) = environment_var {
        Some(environment_var)
    } else {
        None
    }
}

/// Print all environment variables
pub fn print_environment_vars() {
    let vars: std::env::Vars = std::env::vars();

    debug!("Environment Variables:");
    for (key, var) in vars {
        debug!("{key}: {var}");
    }
}

/// Convert's Rust String to C String
///
/// # Errors
///
/// May return a `NulError` if the Rust string contained a nul byte anywhere other than the very end of the string
///
/// # Panics
///
/// May panic if result passes the validity check and somehow fails to unwrap anyway
pub fn get_c_string_from_rust<T: AsRef<str>>(rstr: T) -> Result<*const c_char, NulError>
where
    Vec<u8>: From<T>,
{
    let cstr_result: Result<CString, NulError> = CString::new(rstr);
    if let Ok(cstr) = cstr_result {
        return Ok(cstr.as_ptr());
    }

    Err(cstr_result.err().unwrap())
}
