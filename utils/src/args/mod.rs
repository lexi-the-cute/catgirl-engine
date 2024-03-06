use clap::Parser;
use core::ffi::{c_char, c_int};
use std::sync::OnceLock;

static ARGS: OnceLock<Args> = OnceLock::new();

#[derive(Parser, Debug, Copy, Clone)]
#[command(author, about, long_about = None)]
/// List of possible command line arguments
pub struct Args {
    /// Start the engine in dedicated server mode
    #[arg(short, long, default_value_t = false)]
    pub server: bool,

    /// Display version and copyright info
    #[arg(short, long, default_value_t = false)]
    pub version: bool,
}

/// Parse arguments from C and send to the Clap library
///
/// # Safety
///
/// This only checks if argv is null,
/// it does not verify that argv points to valid data
pub unsafe fn parse_args_from_c(argc: c_int, argv: *const *const c_char) {
    use core::ffi::CStr;

    // Check if argv is null
    if argv.is_null() {
        return;
    }

    // Parse array out of argv
    let c_args: &[*const c_char] = unsafe { std::slice::from_raw_parts(argv, argc as usize) };

    let mut args: Vec<String> = vec![];
    for &arg in c_args {
        let c_str: &CStr = unsafe { CStr::from_ptr(arg) };
        let str_slice: &str = c_str.to_str().unwrap();

        args.push(str_slice.to_owned());
    }

    let _ = ARGS.set(Args::parse_from(args.iter()));
}

pub fn get_args() -> Option<Args> {
    ARGS.get().copied()
}
