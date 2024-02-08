pub mod game_loop;

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{Builder, JoinHandle};
use clap::Parser;

#[cfg(feature = "server")]
use crate::server;

#[cfg(target_os = "android")]
use std::sync::OnceLock;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
pub(crate) static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

// Constants
pub const NAME: &str = "Catgirl Engine";

#[allow(dead_code)]
pub const TAG: &str = "CatgirlEngine";

pub struct ThreadsStruct {
    #[cfg(feature = "server")]
    server: JoinHandle<()>
}

pub struct ChannelStruct {
    sender: Option<Sender<()>>,
    receiver: Option<Receiver<()>>
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Server
    #[arg(short, long, default_value_t = false)]
    server: bool
}

pub fn get_args() -> Args {
    return Args::parse();
}

pub(crate) fn launch() -> isize {
    match start() {
        Ok(_) => {
            return 0;
        }
        Err(_error) => {
            return -1;
        }
    }
}

#[cfg(all(target_os = "android", feature = "client"))]
pub(crate) fn start_android(app: AndroidApp) -> Result<(), String> {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);

    return start();
}

pub(crate) fn setup_logger() {
    if cfg!(target_os = "android") {
        // Limited Filter: trace,android_activity=debug,winit=debug
        // Stronger Filter: trace,android_activity=off,winit=off

        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag(TAG)
                .with_filter(
                    android_logger::FilterBuilder::new()
                        .parse("trace,android_activity=off,winit=off")
                        .build(),
                ),
        );
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(string) = info.payload().downcast_ref::<String>() {
            error!("Caught panic: {:?}", string);
        }
    }));
}

fn start() -> Result<(), String> {
    // let (tx, rx) = mpsc::channel();
    info!("Starting Game...");

    debug!("Setting panic hook...");
    set_panic_hook();

    /* This is a server/client model
     *
     * The server will be solely loaded on a standalone server.
     *
     * The client can either run standalone (multiplayer)
     *   or run both at the same time (singleplayer).
     */
    #[cfg(feature = "server")]
    let (sptx, sprx) = mpsc::channel::<()>(); // Physics Messages Send

    #[cfg(feature = "server")]
    let (rptx, rprx) = mpsc::channel::<()>(); // Physics Messages Receive

    // Treat As If Physical Server (Player Movement)
    #[cfg(feature = "server")]
    let physics_thread: JoinHandle<()> = Builder::new()
        .name("physics".to_string())
        .spawn(|| server::start(rptx, sprx))
        .unwrap(); // Physics

    debug!("Starting Main Loop...");

    let threads: ThreadsStruct = ThreadsStruct {
        #[cfg(feature = "server")]
        server: physics_thread
    };

    let channels: ChannelStruct = ChannelStruct {
        sender: Some(sptx),
        receiver: Some(rprx)
    };

    // TODO: Implement check with command line args and/or config
    if cfg!(not(feature = "client")) || get_args().server {
        game_loop::headless_loop(threads, channels);
    } else {
        game_loop::gui_loop(threads, channels);
    }

    Ok(())
}