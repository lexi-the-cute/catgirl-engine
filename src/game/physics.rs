use std::sync::atomic::{AtomicI8, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::sync::{Mutex, OnceLock, MutexGuard};
use std::time::Duration;

static LOOPSTRUCT: OnceLock<PhysicsLoopStruct> = OnceLock::new();

extern "C" {
    // emscripten_set_main_loop_arg(em_arg_callback_func func, void *arg, int fps, int simulate_infinite_loop)
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    pub fn emscripten_set_main_loop(
        func: extern "C" fn() -> bool,
        fps: std::os::raw::c_int,
        simulate_infinite_loop: std::os::raw::c_int
    );
}

#[derive(Debug)]
#[repr(C)]
pub struct PhysicsLoopStruct {
    pub receive: Mutex<Receiver<()>>,  // Receive From Main Thread In Physics Thread
    pub i: AtomicI8
}

// This thread handles physics (aka the server)
pub fn start(tx: Sender<()>, rx: Receiver<()>) {
    run(rx).map_err(|err: String| {
        error!("Physics Crash: {:?}", err);
    }).ok();

    tx.send(()).ok();
}

fn run(rx: Receiver<()>) -> Result<(), String> {
    let loopstruct: PhysicsLoopStruct = PhysicsLoopStruct {
        receive: rx.into(), i: AtomicI8::new(0)
    };

    LOOPSTRUCT.set(loopstruct).unwrap();

    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    unsafe {
        emscripten_set_main_loop(physics_loop, -1, 0);
    }
    
    #[cfg(not(all(target_family="wasm", target_os="emscripten")))]
    loop {
        let exit_loop: bool = physics_loop();
        if exit_loop {
            // Ending Loop
            break;
        }
    }

    Ok(())
}

fn should_terminate_thread(loopstruct: &PhysicsLoopStruct) -> bool {
    let rx: MutexGuard<Receiver<()>> = loopstruct.receive.lock().unwrap();

    match rx.try_recv() {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

extern "C" fn physics_loop() -> bool {
    let loopstruct: &PhysicsLoopStruct = LOOPSTRUCT.get().unwrap();

    if should_terminate_thread(loopstruct) {
        debug!("Terminating Physics Thread...");
        return true;
    }
    
    loopstruct.i.fetch_add(1, Ordering::Relaxed);

    if loopstruct.i.load(Ordering::Relaxed) > 60 {
        loopstruct.i.store(0, Ordering::Relaxed);
    }

    update(loopstruct);
    return false;
}

fn update(_loopstruct: &PhysicsLoopStruct) {
    // debug!("Physics Update: {}", _loopstruct.i.load(Ordering::Relaxed));

    // Slow Down Physics (60 FPS)
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}