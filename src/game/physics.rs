use std::thread;
use std::time::Duration;

// This thread handles physics (aka the server)
pub fn start() {
    let mut i: i8 = 0;

    loop {
        i += 1;

        if i > 60 {
            i = 0;
        }

        update(i);
    }
}

fn update(_i: i8) {
    // debug!("Physics Update: {}", i);

    // Slow Down Physics (60 FPS)
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}