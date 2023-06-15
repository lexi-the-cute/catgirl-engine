//! Some utility functions for the web browser
//! 
//! This file provides functions which will be useful
//! for interacting with a web browser

#[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
use std::os::raw::{c_char, c_int};

#[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
use std::ffi::CStr;

// Interesting Links
// https://users.rust-lang.org/t/fast-removing-chars-from-string/24554/3
// https://internals.rust-lang.org/t/justification-for-rust-not-supporting-function-overloading-directly/7012/2

/// cbindgen:ignore
#[allow(unused_doc_comments)]
#[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
extern "C" {
    fn emscripten_run_script(script: *const c_char);
    fn emscripten_run_script_int(script: *const c_char) -> c_int;
    fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
}

/// Allows calling Javascript inline from Rust
/// 
/// Example:
/// ```rust
/// #[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
/// browser::run_script("console.log(GL.currentContext);");
/// ```
#[allow(dead_code)]
#[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
pub fn run_script(script: &str) {
    unsafe {
        emscripten_run_script(format!("{}\0", script).as_ptr() as *const c_char);
    }
}

/// Allows calling Javascript inline from Rust
/// with integer as return
/// 
/// Example:
/// ```rust
/// #[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
/// debug!("{}", browser::run_script_int("(function() { return 2+2 })();"));
/// ```
#[allow(dead_code)]
#[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
pub fn run_script_int(script: &str) -> i32 {
    unsafe {
        return emscripten_run_script_int(format!("{}\0", script).as_ptr() as *const c_char);
    }
}

/// Allows calling Javascript inline from Rust
/// with string as return
/// 
/// Example:
/// ```rust
/// #[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
/// debug!("{}", browser::run_script_string("(function() { return 'Hello ' + 'world'; })();"));
/// ```
#[allow(dead_code)]
#[cfg(all(target_family="wasm", target_os="emscripten", feature="browser"))]
pub fn run_script_string(script: &str) -> &str {
    unsafe {
        let result: *const c_char = emscripten_run_script_string(format!("{}\0", script).as_ptr() as *const c_char);

        return CStr::from_ptr(result).to_str().unwrap();
    }
}