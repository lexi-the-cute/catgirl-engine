#![feature(start)]

#[macro_use]
extern crate log;

mod client;
mod game;
mod server;

// Run as Executable (e.g. Linux)
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    game::setup_logger();
    debug!("Launched as binary...");

    return game::launch(_argc, _argv);
}