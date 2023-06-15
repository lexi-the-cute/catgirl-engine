#![allow(unused_imports)]

use std::thread;
use std::thread::JoinHandle;
use std::sync::{Mutex, OnceLock, MutexGuard};
use std::sync::mpsc::{self, Sender, Receiver};

mod physics;
mod render;
mod entity;

static LOOPSTRUCT: OnceLock<MainLoopStruct> = OnceLock::new();

/// cbindgen:ignore
#[allow(unused_doc_comments)]
extern "C" {
    // emscripten_set_main_loop_arg(em_arg_callback_func func, void *arg, int fps, int simulate_infinite_loop)
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    fn emscripten_set_main_loop(
        func: extern "C" fn() -> bool,
        fps: std::os::raw::c_int,
        simulate_infinite_loop: std::os::raw::c_int
    );
}

#[derive(Debug)]
#[repr(C)]
struct MainLoopStruct {
    // Physics Messages Send
    #[cfg(feature="server")]
    sptx: Mutex<Sender<()>>,  // Send To Physics Thread From Main Thread
    // sprx: Mutex<Receiver<()>>,  // Receive From Main Thread In Physics Thread

    // Render Messages Send
    #[cfg(feature="client")]
    srtx: Mutex<Sender<()>>,  // Send To Render Thread From Main Thread
    // srrx: Mutex<Receiver<()>>,  // Receive From Main Thread In Render Thread

    // Physics Messages Receive
    // rptx: Mutex<Sender<()>>,  // Send To Main Thread From Physics Thread
    #[cfg(feature="server")]
    rprx: Mutex<Receiver<()>>,  // Receive From Physics Thread In Main Thread

    // Render Messages Receive
    // rrtx: Mutex<Sender<()>>,  // Send To Main Thread From Render Thread
    #[cfg(feature="client")]
    rrrx: Mutex<Receiver<()>>,  // Receive From Render Thread In Main Thread

    // Server Thread (Physics)
    #[cfg(feature="server")]
    physics_thread: Mutex<JoinHandle<()>>,

    // Client Thread (Render)
    #[cfg(feature="client")]
    render_thread: Mutex<JoinHandle<()>>
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
    #[cfg(feature="server")]
    let (sptx, sprx) = mpsc::channel::<()>();  // Physics Messages Send

    #[cfg(feature="client")]
    let (srtx, srrx) = mpsc::channel::<()>();  // Render Messages Send

    #[cfg(feature="server")]
    let (rptx, rprx) = mpsc::channel::<()>();  // Physics Messages Receive

    #[cfg(feature="client")]
    let (rrtx, rrrx) = mpsc::channel::<()>();  // Render Messages Receive

    // Treat As If Physical Server (Player Movement)
    #[cfg(feature="server")]
    let physics_thread: JoinHandle<()> = thread::Builder::new().name("physics".to_string())
                    .spawn(|| physics::start(rptx, sprx)).unwrap();  // Server

    // Treat As If Physical Client (User Input)
    #[cfg(feature="client")]
    let render_thread: JoinHandle<()> = thread::Builder::new().name("render".to_string())
                    .spawn(|| render::start(rrtx, srrx)).unwrap();  // Client

    let loopstruct: MainLoopStruct = MainLoopStruct {
        #[cfg(feature="server")]
        sptx: sptx.into(),

        #[cfg(feature="server")]
        rprx: rprx.into(),

        #[cfg(feature="client")]
        srtx: srtx.into(),
        
        #[cfg(feature="client")]
        rrrx: rrrx.into(),

        #[cfg(feature="server")]
        physics_thread: physics_thread.into(),

        #[cfg(feature="client")]
        render_thread: render_thread.into()
    };

    LOOPSTRUCT.set(loopstruct).unwrap();

    debug!("Starting Main Loop...");
    /*
     * Intentionally not targetting feature "browser" here
     *   as emscripten is multi-platform.
     */
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    unsafe {
        emscripten_set_main_loop(main_loop, 0, 1);
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

#[cfg(any(feature="server", feature="client"))]
fn is_finished(loopstruct: &MainLoopStruct) -> bool {
    #[cfg(feature="server")]
    let physics_thread: MutexGuard<JoinHandle<()>> = loopstruct.physics_thread.lock().unwrap();

    #[cfg(feature="client")]
    let render_thread: MutexGuard<JoinHandle<()>> = loopstruct.render_thread.lock().unwrap();

    #[cfg(all(feature="server", feature="client"))]
    return physics_thread.is_finished() && render_thread.is_finished();

    #[cfg(all(not(feature="client"), feature="server"))]
    return physics_thread.is_finished();

    #[cfg(all(not(feature="server"), feature="client"))]
    return render_thread.is_finished();
}

#[cfg(feature="server")]
fn is_physics_thread_terminated(loopstruct: &MainLoopStruct) -> bool {
    let receiver: MutexGuard<Receiver<()>> = loopstruct.rprx.lock().unwrap();

    #[cfg(feature="client")]
    let sender: MutexGuard<Sender<()>> = loopstruct.srtx.lock().unwrap();

    match receiver.try_recv() {
        Ok(_) => {
            #[cfg(feature="client")]
            sender.send(()).ok();

            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

#[cfg(feature="client")]
fn is_render_thread_terminated(loopstruct: &MainLoopStruct) -> bool {
    let receiver: MutexGuard<Receiver<()>> = loopstruct.rrrx.lock().unwrap();

    #[cfg(feature="server")]
    let sender: MutexGuard<Sender<()>> = loopstruct.sptx.lock().unwrap();

    match receiver.try_recv() {
        Ok(_) => {
            #[cfg(feature="server")]
            sender.send(()).ok();

            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

extern "C" fn main_loop() -> bool {
    #[allow(unused_variables)]
    let loopstruct: &MainLoopStruct = LOOPSTRUCT.get().unwrap();

    #[cfg(any(feature="server", feature="client"))]
    if is_finished(loopstruct) {
        info!("Stopping Game...");
        return true;
    }

    #[cfg(feature="server")]
    if is_physics_thread_terminated(loopstruct) {
        debug!("Physics Thread Terminated...");
    }

    #[cfg(feature="client")]
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

    #[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
    crate::loggers::browser::init().unwrap();

    #[cfg(not(any(target_os="android", target_family="wasm")))]
    // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
    pretty_env_logger::init();
}