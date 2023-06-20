#[macro_use] extern crate log;

mod game;
mod server;
mod client;

// Run as Library (e.g. Android and WebAssembly)
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn main() {
    game::start();
}