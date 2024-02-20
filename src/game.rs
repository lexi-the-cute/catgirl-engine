use build_info::{chrono::Datelike, BuildInfo, CrateInfo};
use clap::Parser;

// Constants
#[cfg(target_os = "android")]
pub const TAG: &str = "CatgirlEngine";

// Generate build_info() function at compile time
build_info::build_info!(fn build_info);

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub(crate) struct Args {
    /// Start the engine in dedicated server mode
    #[arg(short, long, default_value_t = false)]
    server: bool,

    /// Display version and copyright info
    #[arg(short, long, default_value_t = false)]
    pub(crate) version: bool,
}

#[no_mangle]
pub fn get_args() -> Args {
    Args::parse()
}

// TODO: Implement non-recursively
// fn print_dependencies(info: &BuildInfo) {
//     let mut dependency_list: Vec<&build_info::CrateInfo> = vec![];
//     let mut temp_dependency_list = vec![info.crate_info.dependencies.iter()];

//     let loop_dependencies: bool = true;
//     while loop_dependencies {
//         let next: Option<&build_info::CrateInfo> = temp_dependency_list.pop().unwrap().next();

//         if next.is_none() {
//             break;
//         }

//         let dep: &build_info::CrateInfo = next.unwrap();
//         dependency_list.push(dep);
//         temp_dependency_list.push(dep.dependencies.iter())
//     }

//     for dep in dependency_list.iter() {
//         println!("{} v{} - License {}", dep.name, dep.version, dep.license.as_ref().unwrap())
//     }
// }

// Recursive implementation
fn print_dependencies(info: &BuildInfo) {
    let crate_info = &info.crate_info;

    let dependency_list = recurse_dependencies(crate_info);

    for dep in dependency_list.iter() {
        println!(
            "{} v{} - License {}",
            dep.name,
            dep.version,
            dep.license.as_ref().unwrap()
        );
    }
}

// TODO: Work on this when not tired
fn recurse_dependencies(crate_info: &CrateInfo) -> Vec<&CrateInfo> {
    let mut dependency_list: Vec<&CrateInfo> = vec![];

    for dep in crate_info.dependencies.iter() {
        let mut exists: bool = false;
        for entry in &dependency_list {
            if entry.name == dep.name {
                exists = true;
                break;
            }
        }

        if !exists {
            dependency_list.push(dep);
        }

        // for entry in recurse_dependencies(dep) {

        // }
    }

    dependency_list.clone()
}

pub(crate) fn print_version() {
    let info: &BuildInfo = build_info();

    // The $... are proc macros - https://doc.rust-lang.org/reference/procedural-macros.html
    // Example: catgirl-engine v0.6.0 built with rustc 1.76.0 (07dca489a 2024-02-04) at 2024-02-20 07:40:40Z
    println!(
        "{} v{} built with {} at {}",
        info.crate_info.name, info.crate_info.version, info.compiler, info.timestamp
    );

    // Example: Copyright (C) 2024 Alexis <@alexis@foxgirl.land> - Zlib License
    println!(
        "Copyright (C) {} {} - {} License",
        info.timestamp.year(),
        info.crate_info.authors[0],
        info.crate_info.license.as_ref().unwrap()
    );
    println!();

    // Print all dependencies
    print_dependencies(info);
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
        if let Err(error) = console_log::init_with_level(tracing::log::Level::Debug) {
            warn!("Failed to initialize console logger...")
        }
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}

#[cfg(feature = "tracing-subscriber")]
pub(crate) fn setup_tracer() {
    // Construct a subscriber to print formatted traces to stdout
    let subscriber: tracing_subscriber::FmtSubscriber =
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(false)
            .finish();

    // Process future traces
    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        warn!(
            "Failed to set the tracing subscriber as the global default... Message: {:?}",
            error
        )
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
        })
        .expect("Could not create Interrupt Handler (e.g. Ctrl+C)...");
    }

    debug!("Starting Main Loop...");

    #[cfg(feature = "server")]
    if cfg!(not(feature = "client")) || get_args().server {
        return server::game_loop::game_loop();
    }

    #[cfg(feature = "client")]
    return client::game_loop::game_loop();
}
