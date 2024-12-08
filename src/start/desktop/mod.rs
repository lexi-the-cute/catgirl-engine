// For finding target info: https://stackoverflow.com/a/76480973
#![cfg(any(target_family = "unix", target_family = "windows"))]
#![cfg(not(target_os = "android"))]
#![cfg(not(target_os = "ios"))]

use core::ffi::{c_char, c_int};

// Run as Library
/// Catgirl Engine start
///
/// The starting point when calling as a desktop library
#[unsafe(no_mangle)]
pub extern "C" fn start_engine(argc: c_int, argv: *const *const c_char) -> c_int {
    use crate::{build, setup};

    // Setup logger for debugging
    #[cfg(feature = "logging-subscriber")]
    setup::setup_logger();

    // Transfers embedded resources into utility crate
    utils::resources::store_embedded_resources(crate::resources::get_embedded_resources());

    // Helps with license compliance
    build::license_compliance_helper();

    // Create a vector of args from C styled args
    // We create a new pointer so we guarantee the pointer we are passing is valid
    // This doesn't say anything about the underlying data, but that's the responsibility of
    //   parse_args_from_c(...) to validate
    let args: Option<Vec<String>>;
    unsafe {
        args = utils::args::c_abi::parse_args_from_c(argc, argv.cast::<*const *const c_char>());
    }

    // Override Clap's args
    if let Some(args) = args {
        utils::args::set_parsed_args(args);
    }

    // Print version and copyright info
    if setup::get_args().version {
        build::print_version();
        build::print_build_info();
        build::print_dependencies();

        println!();
        build::print_license();
        return 0;
    }

    // Process args for future use
    setup::process_args();

    debug!("Launched as library...");
    build::log_build_info();

    match setup::start() {
        Err(error) => {
            error!("{:?}", error);

            1
        }
        _ => 0,
    }
}
