// #![cfg(any(target_family = "unix", target_family = "windows"))]
// #![cfg(not(target_os = "android"))]
// #![cfg(not(target_os = "ios"))]

/// Handles generic version of Args
mod generic;

/// Handles Linux version of Args
mod linux;

/// Handles parsing from C ABI
pub mod c_abi;

use std::sync::OnceLock;

#[cfg(not(target_os = "linux"))]
pub use generic::Args;

#[cfg(target_os = "linux")]
pub use linux::Args;

/// Reference to command line args specified by function
static ARGS: OnceLock<Args> = OnceLock::new();

/// Set parsed args passed in from function
pub fn set_parsed_args(args: Vec<String>) {
    use clap::Parser;

    // If we already set the args, don't save again
    // It's a OnceLock, we can only set it once anyway
    if ARGS.get().is_some() {
        return;
    }

    let _ = ARGS.set(Args::parse_from(args.iter()));
}

/// Retrieve parsed args previously passed in from function
pub fn get_args() -> Option<Args> {
    ARGS.get().cloned()
}
