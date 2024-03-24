//! Utilities for the catgirl-engine crate

#![warn(missing_docs)]

use std::env;

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

/// Get current time in seconds
///
/// # Panics
///
/// Will cause panic on wasm and potentially if clock went backwards.
/// TODO: Will be replacing with Instant in the nearby future.
#[must_use]
pub fn get_current_time_seconds() -> u64 {
    #[cfg(target_family = "wasm")]
    {
        (js_sys::Date::now() / 1000.0).to_bits()
    }

    #[cfg(not(target_family = "wasm"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
