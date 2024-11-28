//! Starting point for the game engine as a binary
//!
//! This is a game engine designed for moddability

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
mod setup;

/// Module for storing and using build data
mod build;

/// Module for storing resources
mod resources;

/// Run as Executable (e.g. Linux)
///
/// # Errors
///
/// May return a `String` if an error propagated up the stack
fn main() -> Result<(), String> {
    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    #[cfg(feature = "embed-resources")]
    utils::resources::store_embedded_resources(resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

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
