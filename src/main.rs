#[macro_use]
extern crate tracing;

mod game;

// TODO: Split client, server, and common into 3 different crates
// Run as Executable (e.g. Linux)
fn main() {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    // Print version and copyright info
    if game::get_args().version {
        game::print_version();
        return;
    }

    game::setup_logger();
    debug!("Launched as binary...");

    if let Err(error) = game::start() {
        error!("{:?}", error)
    }
}
