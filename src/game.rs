use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{Builder, JoinHandle};
use clap::Parser;

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

// TODO: Replace this...
pub struct ThreadsStruct {
    #[cfg(feature = "server")]
    pub server: JoinHandle<()>
}

// TODO: Replace this...
pub struct ChannelStruct {
    pub sender: Option<Sender<()>>,
    pub receiver: Option<Receiver<()>>
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Server
    #[arg(short, long, default_value_t = false)]
    server: bool
}

#[no_mangle]
pub fn get_args() -> Args {
    Args::parse()
}

#[cfg(all(target_os = "android", feature = "client"))]
pub(crate) fn start_android(app: AndroidApp) -> Result<(), String> {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);

    start()
}

pub(crate) fn setup_logger() {
    if cfg!(target_os = "android") {
        // Limited Filter: trace,android_activity=debug,winit=debug
        // Stronger Filter: trace,android_activity=off,winit=off

        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(tracing::log::LevelFilter::Trace)
                .with_tag(TAG)
                .with_filter(
                    android_logger::FilterBuilder::new()
                        .parse("trace,android_activity=off,winit=off")
                        .build(),
                ),
        );
    } else if cfg!(target_arch = "wasm32") {
        #[cfg(target_arch = "wasm32")]
        match console_log::init_with_level(tracing::log::Level::Debug) {
            Err(_) => warn!("Failed to initialize console logger..."),
            _ => ()
        }
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}

#[cfg(feature = "tracing-subscriber")]
pub(crate) fn setup_tracer() {
    // Construct a subscriber to print formatted traces to stdout
    let subscriber: tracing_subscriber::FmtSubscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(false)
        .finish();

    // Process future traces
    match tracing::subscriber::set_global_default(subscriber) {
        Err(error) => {
            warn!("Failed to set the tracing subscriber as the global default... Message: {:?}", error)
        },
        _ => ()
    }
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        if let Some(string) = info.payload().downcast_ref::<String>() {
            error!("Caught panic: {:?}", string);
        }
    }));
}

pub fn start() -> Result<(), String> {
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
    // https://sotrh.github.io/learn-wgpu/beginner/tutorial1-window/#more-code
    #[cfg(feature = "server")]
    let (sptx, sprx) = mpsc::channel::<()>(); // Physics Messages Send

    #[cfg(feature = "server")]
    let (rptx, rprx) = mpsc::channel::<()>(); // Physics Messages Receive

    // Treat As If Physical Server (Player Movement)
    #[cfg(feature = "server")]
    let physics_thread: JoinHandle<()> = Builder::new()
        .name("physics".to_string())
        .spawn(|| crate::common::physics::start(rptx, sprx))
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

    if cfg!(not(feature = "client")) || get_args().server {
        crate::common::game_loop::headless_loop(threads, channels);
    } else {
        crate::client::game_loop::gui_loop(threads, channels);
    }

    Ok(())
}