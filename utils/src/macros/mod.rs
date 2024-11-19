/// Prints strings in a format usable by the target platform
///
/// Will log with info for Wasm while printing a line on other platforms
#[macro_export]
macro_rules! println_string {
    // Single Arg
    ($arg:tt) => {{
        if cfg!(target_family = "wasm") {
            info!("{}", $arg);
        } else {
            println!("{}", $arg);
        }
    }};

    // Multiple Args
    ($fmt:expr, $($arg:tt)+) => {{
        if cfg!(target_family = "wasm") {
            info!($fmt, $($arg)*);
        } else {
            println!($fmt, $($arg)*);
        }
    }};
}
