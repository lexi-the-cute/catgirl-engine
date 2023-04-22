use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;

pub mod physics;
pub mod render;
pub mod entity;

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

    loop {
        if physics_thread.is_finished() && render_thread.is_finished() {
            info!("Stopping Game...");
            break;
        }

        match rprx.try_recv() {
            Ok(_) => {
                debug!("Physics Thread Terminated...");
                srtx.send(()).ok();
            }
            Err(_) => {
                // Not Implemented At The Moment
            }
        }

        match rrrx.try_recv() {
            Ok(_) => {
                debug!("Render Thread Terminated...");
                sptx.send(()).ok();
            }
            Err(_) => {
                // Not Implemented At The Moment
            }
        }
    }

    // std::process::exit(0);
}

fn setup_logger() {
    if cfg!(target_os = "android") {
        android_logger::init_once(
            android_logger::Config::default()
                    .with_max_level(log::LevelFilter::Trace)
                    .with_tag("CatgirlEngine")
        );
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}