#![cfg(feature = "client")]

use std::sync::mpsc::{Receiver, Sender};

pub mod browser;

// This thread handles both rendering and input (aka the client)
pub fn start(tx: Sender<()>, rx: Receiver<()>) {
    run(rx)
        .map_err(|err: String| {
            error!("Client (Render) Crash: {:?}", err);
        })
        .ok();

    tx.send(()).ok();
}

fn run(rx: Receiver<()>) -> Result<(), String> {
    // Rendering Work Goes Here
    // https://github.com/dasifefe/rust-game-development-frameworks#graphics-only
    loop {
        match rx.try_recv() {
            Ok(_) => {
                break;
            }
            Err(_) => {
                // Not Implemented
            }
        }

        // debug!("Render Loop!")
    }

    Ok(())
}
