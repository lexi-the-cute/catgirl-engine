use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver};

pub mod physics;
pub mod render;
pub mod entity;

extern "C" {
    // emscripten_set_main_loop_arg(em_arg_callback_func func, void *arg, int fps, int simulate_infinite_loop)
    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    pub fn emscripten_set_main_loop_arg(
        func: extern "C" fn(*mut LoopStruct) -> bool,
        arg: *mut (),
        fps: std::os::raw::c_int,
        simulate_infinite_loop: std::os::raw::c_int
    );
}

// #[derive(Debug)]
#[repr(C)]
pub struct LoopStruct {
    // Physics Messages Send
    pub sptx: Sender<()>,  // Send To Physics Thread From Main Thread
    // pub sprx: Receiver<()>,  // Receive From Main Thread In Physics Thread

    // Render Messages Send
    pub srtx: Sender<()>,  // Send To Render Thread From Main Thread
    // pub srrx: Receiver<()>,  // Receive From Main Thread In Render Thread

    // Physics Messages Receive
    // pub rptx: Sender<()>,  // Send To Main Thread From Physics Thread
    pub rprx: Receiver<()>,  // Receive From Physics Thread In Main Thread

    // Render Messages Receive
    // pub rrtx: Sender<()>,  // Send To Main Thread From Render Thread
    pub rrrx: Receiver<()>,  // Receive From Render Thread In Main Thread

    // Server Thread (Physics)
    pub physics_thread: JoinHandle<()>,

    // Client Thread (Render)
    pub render_thread: JoinHandle<()>
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

    let mut loop_struct: LoopStruct = LoopStruct {
        sptx, srtx, rprx, rrrx, physics_thread, render_thread
    };

    #[cfg(all(target_family="wasm", target_os="emscripten"))]
    unsafe {
        emscripten_set_main_loop_arg(main_loop, &mut loop_struct as *mut _ as *mut (), -1, 0);
    }
    
    #[cfg(not(all(target_family="wasm", target_os="emscripten")))]
    loop {
        if main_loop(&mut loop_struct) {
            // Ending Loop
            break;
        }
    }

    // std::process::exit(0);
}

extern "C" fn main_loop(loop_struct: *mut LoopStruct) -> bool {
    unsafe {
        if (*loop_struct).physics_thread.is_finished() && (*loop_struct).render_thread.is_finished() {
            info!("Stopping Game...");
            return true;
        }

        match (*loop_struct).rprx.try_recv() { // <---- TODO: Determine why this breaks Emscripten...
            Ok(_) => {
                debug!("Physics Thread Terminated...");
                (*loop_struct).srtx.send(()).ok();
            }
            Err(_) => {
                // Not Implemented At The Moment
            }
        }

        match (*loop_struct).rrrx.try_recv() {
            Ok(_) => {
                debug!("Render Thread Terminated...");
                (*loop_struct).sptx.send(()).ok();
            }
            Err(_) => {
                // Not Implemented At The Moment
            }
        }

        return false;
    }
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