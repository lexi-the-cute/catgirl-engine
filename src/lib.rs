#[macro_use] extern crate log;

use sdl2::libc;
use wasm_bindgen::prelude::*;

mod game;
mod android;

// Run as Library (e.g. Android)
#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    game::start();
    return 0;
}

// Run as Library (e.g. Webassembly)
#[wasm_bindgen(start)]
fn wasm_init() -> Result<(), JsValue> {
    game::start();

    Ok(())
}