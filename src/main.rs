#[macro_use] extern crate log;

mod game;
mod loggers;

// Run as Executable (e.g. Linux)
pub fn main() {
    game::start();
}