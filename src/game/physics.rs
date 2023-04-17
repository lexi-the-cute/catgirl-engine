use std::thread;
use std::time::Duration;

// This thread handles physics (aka the server)
pub fn start() {
    loop {
        update();
    }
}

fn update() {
    for i in 1..10 {
        println!("update: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // thread::sleep(Duration::from_millis(1));
}