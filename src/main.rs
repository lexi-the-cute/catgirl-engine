// https://sunjay.dev/learn-game-dev/opening-a-window.html

use std::thread;
use std::thread::JoinHandle;
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
    let _handle: JoinHandle<()> = thread::spawn(|| physics::start());
    let _handle: JoinHandle<Result<(), String>> = thread::spawn(|| render::start());

    _handle.join().unwrap().map_err(|err: String| println!("{:?}", err)).ok();
}