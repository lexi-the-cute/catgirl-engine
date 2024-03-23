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
pub fn matches_environment_var(key: &str, value: &str) -> bool {
    let environment_var: Result<String, env::VarError> = env::var(key);
    environment_var.is_ok() && environment_var.unwrap() == value
}

/// Get value of the requested environment variable
pub fn get_environment_var(key: &str) -> Option<String> {
    let environment_var: Result<String, env::VarError> = env::var(key);

    if environment_var.is_ok() {
        Some(environment_var.unwrap())
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
