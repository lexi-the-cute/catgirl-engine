use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

// This thread handles physics (aka the server)
pub fn start(tx: Sender<()>, rx: Receiver<()>) {
    run(rx).map_err(|err: String| {
        error!("Physics Crash: {:?}", err);
    }).ok();

    tx.send(()).ok();
}

fn run(rx: Receiver<()>) -> Result<(), String> {
    let mut i: i8 = 0;

    loop {
        match rx.try_recv() {
            Ok(_) => {
                debug!("Terminating Physics Thread...");
                break;
            }
            Err(_) => {
                // Not Implemented At The Moment
            }
        }

        i += 1;

        if i > 60 {
            i = 0;
        }

        update(i);
    }

    Ok(())
}

fn update(_i: i8) {
    // debug!("Physics Update: {}", _i);

    // Slow Down Physics (60 FPS)
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}