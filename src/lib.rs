#![feature(start)]

#[macro_use] extern crate log;

use core::ffi::{c_char, c_int};

mod game;
mod server;
mod client;

// Run as Library (e.g. Android and WebAssembly)
#[start]
#[no_mangle]
#[allow(non_snake_case)]
pub extern fn ce_start(argc: c_int, argv: *const *const c_char) -> c_int {
    game::setup_logger();
    debug!("Launched as library...");
    
    // This is to convert from C Main Args to Rust Main Args
    let _argc: isize = argc.try_into().unwrap();
    let _argv: *const *const u8 = argv as *const *const u8;

    return game::start(_argc, _argv).try_into().unwrap();
}