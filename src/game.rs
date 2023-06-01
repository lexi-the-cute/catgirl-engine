use std::thread;
use std::thread::JoinHandle;
use std::sync::{Mutex, OnceLock, MutexGuard};
use std::sync::mpsc::{self, Sender, Receiver};

pub mod physics;
pub mod render;
pub mod entity;

static LOOPSTRUCT: OnceLock<MainLoopStruct> = OnceLock::new();

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
pub struct MainLoopStruct {
    // Physics Messages Send
    pub sptx: Mutex<Sender<()>>,  // Send To Physics Thread From Main Thread
    // pub sprx: Mutex<Receiver<()>>,  // Receive From Main Thread In Physics Thread

    // Render Messages Send
    pub srtx: Mutex<Sender<()>>,  // Send To Render Thread From Main Thread
    // pub srrx: Mutex<Receiver<()>>,  // Receive From Main Thread In Render Thread

    // Physics Messages Receive
    // pub rptx: Mutex<Sender<()>>,  // Send To Main Thread From Physics Thread
    pub rprx: Mutex<Receiver<()>>,  // Receive From Physics Thread In Main Thread

    // Render Messages Receive
    // pub rrtx: Mutex<Sender<()>>,  // Send To Main Thread From Render Thread
    pub rrrx: Mutex<Receiver<()>>,  // Receive From Render Thread In Main Thread

    // Server Thread (Physics)
    pub physics_thread: Mutex<JoinHandle<()>>,

    // Client Thread (Render)
    pub render_thread: Mutex<JoinHandle<()>>
}

pub fn start() {
    // It's much easier to debug when one can see the log (logcat on Android)
    setup_logger();

    // let (tx, rx) = mpsc::channel();
    info!("Starting Game...");

    /* This is a server/client model
     *
     * The server will only be loaded on a standalone server.
     * 
     * The client can either run standalone (multiplayer)
     *   or run both at the same time (singleplayer).
    */
    let (sptx, sprx) = mpsc::channel::<()>();  // Physics Messages Send
    let (srtx, srrx) = mpsc::channel::<()>();  // Render Messages Send

    let (rptx, rprx) = mpsc::channel::<()>();  // Physics Messages Receive
    let (rrtx, rrrx) = mpsc::channel::<()>();  // Render Messages Receive

    // Treat As If Physical Server (Player Movement)
    let physics_thread: JoinHandle<()> = thread::Builder::new().name("physics".to_string())
                    .spawn(|| physics::start(rptx, sprx)).unwrap();  // Server

    // Treat As If Physical Client (User Input)
    let render_thread: JoinHandle<()> = thread::Builder::new().name("render".to_string())
                    .spawn(|| render::start(rrtx, srrx)).unwrap();  // Client

    let loopstruct: MainLoopStruct = MainLoopStruct {
        sptx: sptx.into(), srtx: srtx.into(),
        rprx: rprx.into(), rrrx: rrrx.into(),
        physics_thread: physics_thread.into(),
        render_thread: render_thread.into()
    };

    LOOPSTRUCT.set(loopstruct).unwrap();

    debug!("Starting Main Loop...");
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    unsafe {
        emscripten_set_main_loop(main_loop, -1, 1);
    }
    
    #[cfg(not(all(target_family="wasm", target_os="emscripten")))]
    loop {
        let exit_loop: bool = main_loop();
        if exit_loop {
            // Ending Loop
            break;
        }
    }
    debug!("Exiting Main Loop...");
}

fn is_finished(loopstruct: &MainLoopStruct) -> bool {
    let physics_thread: MutexGuard<JoinHandle<()>> = loopstruct.physics_thread.lock().unwrap();
    let render_thread: MutexGuard<JoinHandle<()>> = loopstruct.render_thread.lock().unwrap();

    return physics_thread.is_finished() && render_thread.is_finished();
}

fn is_physics_thread_terminated(loopstruct: &MainLoopStruct) -> bool {
    let receiver: MutexGuard<Receiver<()>> = loopstruct.rprx.lock().unwrap();
    let sender: MutexGuard<Sender<()>> = loopstruct.srtx.lock().unwrap();

    match receiver.try_recv() {
        Ok(_) => {
            sender.send(()).ok();

            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

fn is_render_thread_terminated(loopstruct: &MainLoopStruct) -> bool {
    let receiver: MutexGuard<Receiver<()>> = loopstruct.rrrx.lock().unwrap();
    let sender: MutexGuard<Sender<()>> = loopstruct.sptx.lock().unwrap();

    // debug!("Try Receive Render Thread Termination... {:?}", receiver.try_recv());
    match receiver.try_recv() {
        Ok(_) => {
            sender.send(()).ok();

            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

extern "C" fn main_loop() -> bool {
    let loopstruct: &MainLoopStruct = LOOPSTRUCT.get().unwrap();

    if is_finished(loopstruct) {
        info!("Stopping Game...");
        return true;
    }

    if is_physics_thread_terminated(loopstruct) {
        debug!("Physics Thread Terminated...");
    }

    if is_render_thread_terminated(loopstruct) {
        debug!("Render Thread Terminated...");
    }

    return false;
}

fn setup_logger() {
    #[cfg(target_os="android")]
    android_logger::init_once(
        android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("CatgirlEngine")
    );

    #[cfg(all(target_family="wasm", feature="browser"))]
    crate::loggers::browser::init().unwrap();

    #[cfg(not(any(target_os="android",target_family="wasm")))]
    // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
    pretty_env_logger::init();
}