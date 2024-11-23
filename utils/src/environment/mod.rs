use std::{env, ffi::OsStr};

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
    let environment_var: Option<String> = get_environment_var(key);
    environment_var.is_some() && environment_var.unwrap() == value
}

/// Print all environment variables
#[unsafe(no_mangle)]
pub extern "C" fn print_environment_vars() {
    let vars: std::env::Vars = std::env::vars();

    println_string!("Environment Variables:");
    for (key, var) in vars {
        if crate::string::is_likely_secret(key.clone()) {
            println_string!("{}: {}", key, crate::string::mask_string(var));
        } else {
            println_string!("{}: {}", key, var);
        }
    }
}
