use sdl2::libc;
use std::thread;
// use std::sync::mpsc;

mod physics;
mod render;

#[no_mangle]
pub extern fn SDL_main(_argc: libc::c_int, _argv: *const *const libc::c_char) -> libc::c_int {
    // TODO: Cleanup Android Related Code
    main();
    return 0;
}

fn main() {
    // let (tx, rx) = mpsc::channel();
    println!("Starting Game...");

    /* This is a server/client model
     *
     * The server will only be loaded on a standalone server.
     * 
     * The client can either run standalone (multiplayer)
     *   or run both at the same time (singleplayer).
    */
    let _handle = thread::spawn(|| physics::start());
    let _handle = thread::spawn(|| render::start());

    _handle.join().unwrap();
}