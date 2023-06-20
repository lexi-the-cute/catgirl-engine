#[macro_use] extern crate log;

mod game;
mod server;
mod client;

// Run as Executable (e.g. Linux)
pub fn main() {
    game::start();
}