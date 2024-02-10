#[macro_use]
extern crate tracing;

mod game;
mod server;

// Run as Executable (e.g. Linux)
fn main() {
    #[cfg(feature = "tracing-subscriber")]
    game::setup_tracer();

    game::get_args();
    game::setup_logger();
    debug!("Launched as binary...");

    game::launch();
}