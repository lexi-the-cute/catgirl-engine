use std::thread;
use std::thread::JoinHandle;
// use std::sync::mpsc;

pub mod physics;
pub mod render;

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
    let _handle: JoinHandle<()> = thread::spawn(|| physics::start());
    let _handle: JoinHandle<Result<(), String>> = thread::spawn(|| render::start());

    _handle.join().unwrap().map_err(|err: String| error!("Crash: {:?}", err)).ok();
    error!("Post Testing Error Logging...");
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