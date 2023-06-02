// extern crate cc;
extern crate cbindgen;

use std::env::{self, Vars};
use std::path::PathBuf;
use cbindgen::{Config, Language};

fn main() {
    // For some reason, the cfg!() macros won't cooperate, so Alexis is doing this herself
    let target_family: String = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let target_os: String = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Only Emscripten builds need the javascript generation flag set
    if target_family.contains("wasm") && target_os == "emscripten" {
        #[cfg(feature = "browser")]
        add_browser_support();
        create_emscripten_wasm();
    }

    // Bindings are only usable when building libs
    create_bindings();
}

#[allow(dead_code)]
fn add_browser_support() {
    let lib_path: PathBuf = PathBuf::from("c").join("browser.c");

    // println!("cargo:rustc-link-search=/path/to/lib");
    // println!("cargo:rustc-link-lib=SDL");
    println!("cargo:rerun-if-changed={}", lib_path.to_str().unwrap());

    cc::Build::new()
        .file(lib_path.to_str().unwrap())
        .compile("browser");
}

fn create_emscripten_wasm() {
    // This is only to run for the wasm32-unknown-emscripten target
    // println!("cargo:warning=Building Emscripten Wasm");
    let parent_dir: PathBuf = target_dir().join("wasm");
    let output_file: String = parent_dir.join(format!("{}.{}", "main", "js"))
    .display()
    .to_string();

    // Create Parent Directories If Not Exists
    std::fs::create_dir_all(parent_dir).unwrap();

    // Flags to Make Emscripten Compile This Correctly (Combined With RUSTFLAGS)
    // https://github.com/emscripten-core/emscripten/blob/main/src/settings.js
    println!("cargo:rustc-env=EMCC_CFLAGS=-O3 -pthread -s STRICT_JS=1 \
                -s WASM=1 \
                -s WASM_BIGINT=1 \
                -s SUPPORT_BIG_ENDIAN=1 \
                -s ALLOW_MEMORY_GROWTH=1 \
                -s SHARED_MEMORY=1 \
                -s ABORT_ON_WASM_EXCEPTIONS=0 \
                -s WASM_WORKERS=1 \
                -s WASMFS=1 \
                -s MINIMAL_RUNTIME_STREAMING_WASM_INSTANTIATION=1 \
                -s TRUSTED_TYPES=1 \
                -s ASSERTIONS=1 \
                -s PTHREADS_DEBUG=0 \
                -s RUNTIME_DEBUG=0 \
                -s ALLOW_BLOCKING_ON_MAIN_THREAD=0 \
                -s PTHREAD_POOL_SIZE=3 \
                -s OFFSCREEN_FRAMEBUFFER=1 \
                -s OFFSCREENCANVAS_SUPPORT=1 \
                -lSDL2 \
                -lSDL2_image \
                -lSDL2_ttf \
                -s EXPORTED_FUNCTIONS=\"['_SDL_main', '_malloc']\"
            ");
    
    println!("cargo:rustc-link-arg=-o{output_file}");
}

fn create_bindings() {
    let crate_directory: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name: String = env::var("CARGO_PKG_NAME").unwrap();

    create_binding("h", Language::C, &package_name, &crate_directory);
    create_binding("hpp", Language::Cxx, &package_name, &crate_directory);
    create_binding("pyx", Language::Cython, &package_name.replace("-", "_"), &crate_directory);
}

fn create_binding(extension: &str, language: Language, package_name: &String, crate_directory: &String) {
    let output_file: String = target_dir()
    .join("binding")
    .join(format!("{}.{}", package_name, extension))
    .display()
    .to_string();

    let mut header: String = 
        "/*\n".to_owned() +
        " * This file exists to help facilitate modding this catgirl game engine...\n" + 
        " * These generated bindings are either public domain or Unlicense where public domain does not exist\n" + 
        " */";
    if language == Language::Cython {
        header =
            "# This file exists to help facilitate modding this catgirl game engine...\n".to_owned() +
            "# These generated bindings are either public domain or Unlicense where public domain does not exist";
    }

    let config: Config = Config {
        namespace: Some(String::from("ffi")),
        header: Some(String::from(header)),
        language: language,
        only_target_dependencies: true,
        no_includes: if language == Language::Cython { true } else { false },
        ..Default::default()
    };

    cbindgen::generate_with_config(&crate_directory, config)
        .unwrap()
        .write_to_file(&output_file);
}

/// Find the location of the `target/` directory. Note that this may be 
/// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR` 
/// variable.
fn target_dir() -> PathBuf {
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target)
    } else {
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
    }
}

#[allow(dead_code)]
fn print_environment_vars() {
    let vars: Vars = std::env::vars();

    for (key, var) in vars {
        println!("cargo:warning=EV: {key}: {var}");
    }
}