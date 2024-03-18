//! Starting point for the catgirl-engine as a binary
//!
//! This is a game engine designed for moddability

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
pub mod setup;

// Run as Executable (e.g. Linux)
fn main() {
    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        setup::print_dependencies();
        return;
    }

    // Setup logger for debugging
    setup::setup_logger();
    debug!("Launched as binary...");

    // Process args for future use
    setup::process_args();

    if let Err(error) = setup::start() {
        error!("{:?}", error);
    }
}
