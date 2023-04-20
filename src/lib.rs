#[macro_use] extern crate log;

use sdl2::libc;

mod game;
mod android;

// Run as Library (e.g. Android)
#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    game::start();
    return 0;
}