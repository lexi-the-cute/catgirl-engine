#![cfg(feature="server")]

use std::sync::mpsc::{Sender, Receiver};

// This thread handles physics (aka the server)
pub fn start(tx: Sender<()>, rx: Receiver<()>) {
    run(rx).map_err(|err: String| {
        error!("Server (Physics) Crash: {:?}", err);
    }).ok();

    tx.send(()).ok();
}

fn run(rx: Receiver<()>) -> Result<(), String> {
    let mut i: i32 = 0;

    // Physics Work Goes Here
    // https://github.com/dasifefe/rust-game-development-frameworks#frameworks-for-physics-and-linear-math-for-2d-and-3d-programming
    loop {
        match rx.try_recv() {
            Ok(_) => {
                break;
            }
            Err(_) => {
                // Not Implemented
            }
        }

        i += 1;

        if i >= 60 {
            i = 0;
        }

        // debug!("{i}")
    }

    Ok(())
}