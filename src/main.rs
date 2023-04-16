// https://sunjay.dev/learn-game-dev/opening-a-window.html

use std::thread;
// use std::sync::mpsc;

mod physics;
mod render;

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