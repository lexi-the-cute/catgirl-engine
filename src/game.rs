#[cfg(feature = "client")]
use client;

#[cfg(feature = "server")]
use server;

use clap::Parser;

// Constants
#[cfg(target_os = "android")]
pub const TAG: &str = "CatgirlEngine";

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

    // Allows handling properly shutting down with SIGINT
    #[cfg(not(target_family = "wasm"))]
    {
        debug!("Setting SIGINT hook...");
        ctrlc::set_handler(move || {
            debug!("SIGINT (Ctrl+C) Was Called! Stopping...");
            std::process::exit(0);
        }).expect("Could not create Interrupt Handler (e.g. Ctrl+C)...");
    }

    debug!("Starting Main Loop...");

    #[cfg(feature = "server")]
    if cfg!(not(feature = "client")) || get_args().server {
        return server::game_loop::game_loop();
    }

    #[cfg(feature = "client")]
    return client::game_loop::game_loop();
}