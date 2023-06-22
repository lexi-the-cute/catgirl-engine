#![feature(start)]

#[macro_use] extern crate log;

mod game;
mod server;
mod client;

// Run as Library (e.g. Android and WebAssembly)
#[start]
#[allow(non_snake_case)]
pub extern fn main(_argc: isize, _argv: *const *const u8) -> isize {
    game::setup_logger();
    debug!("Launched as library...");
    
    return game::start(_argc, _argv);
}