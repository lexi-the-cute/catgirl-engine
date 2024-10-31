//! Starting point for the catgirl-engine as a binary
//!
//! This is a game engine designed for moddability

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Prepare the game engine for running
pub mod setup;

/// Run as Executable (e.g. Linux)
pub extern "Rust" fn main() -> Result<(), String> {
    #[cfg(feature = "tracing-subscriber")]
    setup::setup_tracer();

    // Print version and copyright info
    if setup::get_args().version {
        setup::print_version();
        setup::print_dependencies();
        return Ok(());
    }

    // Setup logger for debugging
    setup::setup_logger();

    // Process args for future use
    setup::process_args();
    debug!("Launched as binary...");
    trace!("Built for Arch: {}", setup::build_info().target.cpu.arch);

    if let Err(error) = setup::start() {
        error!("{:?}", error);

        return Err(error);
    }

    Ok(())
}
