use clap::Parser;
use core::ffi::{c_char, c_int};
use std::{path::PathBuf, sync::OnceLock};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Reference to command line args specified by function
static ARGS: OnceLock<Args> = OnceLock::new();

#[derive(Parser, Debug, Clone)]
#[command(author, about, long_about = None)]
/// List of possible command line arguments
pub struct Args {
    /// Start the engine in dedicated server mode
    #[arg(short, long, default_value_t = false)]
    pub server: bool,

    /// Display version and copyright info
    #[arg(short, long, default_value_t = false)]
    pub version: bool,

    /// Set custom assets path
    #[arg(short, long, default_value = "assets")]
    pub assets: PathBuf,

    /// Set custom resources path
    #[arg(short, long, default_value = "resources")]
    pub resources: PathBuf,

    /// Shows the AppImage help arguments
    // https://github.com/clap-rs/clap/discussions/5401
    #[cfg(feature = "appimage")]
    #[arg(long, default_value_t = false)]
    pub appimage_help: bool,

    /// Install the desktop files for launching from the application menu
    #[cfg(target_os = "linux")]
    #[arg(long, default_value_t = false)]
    pub install_desktop_files: bool,

    /// Uninstall the previously installed desktop files
    #[cfg(target_os = "linux")]
    #[arg(long, default_value_t = false)]
    pub uninstall_desktop_files: bool,

    /// Print all environment variables
    #[cfg(not(target_family = "wasm"))]
    #[arg(long, default_value_t = false)]
    pub print_environment_variables: bool,
}

/// Parse arguments from C and send to the Clap library
///
/// # Safety
///
/// This only checks if argv is null,
/// it does not verify that argv points to valid data
///
/// # Panics
///
/// May panic if one of the arguments contains invalid UTF-8 data
#[must_use]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub unsafe fn parse_args_from_c(
    arg_count: c_int,
    arg_vector_pointer: *const *const *const c_char,
) -> Option<Vec<String>> {
    use core::ffi::CStr;

    // Check if argv_pointer is null
    if arg_vector_pointer.is_null() {
        return None;
    }

    // Cast back to *const *const c_char so we can operate on it
    //  now that we passed the Safe API Boundary/Barrier
    let arg_vector: *const *const c_char = arg_vector_pointer.cast::<*const c_char>();

    // Check if argv is null
    if arg_vector.is_null() {
        return None;
    }

    // Parse array out of argv
    #[allow(clippy::cast_sign_loss)]
    let c_args: &[*const c_char] =
        unsafe { std::slice::from_raw_parts(arg_vector, arg_count as usize) };

    let mut args: Vec<String> = vec![];
    for &arg in c_args {
        let c_str: &CStr = unsafe { CStr::from_ptr(arg) };

        // This can cause panic
        let str_slice: &str = c_str.to_str().unwrap();

        args.push(str_slice.to_string());
    }

    Some(args)
}

/// Set parsed args passed in from function
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn set_parsed_args(args: Vec<String>) {
    // If we already set the args, don't save again
    // It's a OnceLock, we can only set it once anyway
    if ARGS.get().is_some() {
        return;
    }

    let _ = ARGS.set(Args::parse_from(args.iter()));
}

/// Retrieve parsed args previously passed in from function
// #[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn get_args() -> Option<Args> {
    ARGS.get().cloned()
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    #[test]
    fn test_parsing_args_from_c() {
        use super::*;

        // Null pointer should be disregarded (e.g. return is None)
        unsafe {
            assert_eq!(parse_args_from_c(999, core::ptr::null()), None);
        }

        // Valid argument passed in (e.g. return is vec!["hello"])
        unsafe {
            // Create C String
            let arg_one: CString = CString::new("hello").unwrap();
            let arg_one_ptr: *const c_char = arg_one.as_ptr();

            // Add C String to array
            let argv: [*const c_char; 1] = [arg_one_ptr];

            // Test Parser
            assert_eq!(
                parse_args_from_c(
                    i32::try_from(argv.len()).unwrap(),
                    argv.as_ptr().cast::<*const *const c_char>()
                ),
                Some(vec!["hello".to_string()])
            );
        }
    }
}
