#[macro_use]
extern crate tracing;

mod game;

// TODO: Split client, server, and common into 3 different crates
// Run as Executable (e.g. Linux)
fn main() {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    game::get_args();
    game::setup_logger();
    debug!("Launched as binary...");

    if let Err(error) = game::start() {
        error!("{:?}", error)
    }
}
