//! Starting point for the catgirl-engine as a binary
//!
//! This is a game engine designed for moddability

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
pub mod setup;

/// Module for storing and using build data
pub mod build;

/// Run as Executable (e.g. Linux)
///
/// # Errors
///
/// May return a `String` if an error propagated up the stack
pub fn main() -> Result<(), String> {
    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        build::print_version();
        build::print_build_info();
        build::print_dependencies();

        println!();
        build::print_license();
        return Ok(());
    }

    // Process args for future use
    setup::process_args();

    debug!("Launched as binary...");
    build::log_build_info();

    if let Err(error) = setup::start() {
        error!("{:?}", error);

        return Err(error);
    }

    Ok(())
}
