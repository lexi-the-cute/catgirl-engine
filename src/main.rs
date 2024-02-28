//! Starting point for the catgirl-engine as a binary
//!
//! This is a game engine designed for moddability

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

#[cfg(test)]
/// Handles testing the code
mod test;

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
mod setup;

// Run as Executable (e.g. Linux)
fn main() {
    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        return;
    }

    setup::setup_logger();
    debug!("Launched as binary...");

    if let Err(error) = setup::start() {
        error!("{:?}", error)
    }
}
