use std::process::Command;
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();

    // // note that there are a number of downsides to this approach, the comments
    // // below detail how to improve the portability of these commands.
    // Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
    //                    .arg(&format!("{}/hello.o", out_dir))
    //                    .status().unwrap();
    // Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
    //                   .current_dir(&Path::new(&out_dir))
    //                   .status().unwrap();

    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=hello");

    build_apk();
}

fn build_apk() {
    // Modify For Your Needs
    let build_mode = "release";
    let tags = ["aarch64-linux-android", "armv7-linux-androideabi", "i686-linux-android"];

    // Executables
    // See https://developer.android.com/ndk/downloads
    let ndk_build = "/home/alexis/Android/Sdk/ndk/25.2.9519653/ndk-build";
    

    let lib_name = "libmain.so";
    let jni_libs: PathBuf = ["android", "app", "src", "main", "jniLibs"].iter().collect();
    
    compile_sdl_ndk(ndk_build);

    for tag in tags {
        make_deps_dir(tag, build_mode);
    }
}

fn compile_sdl_ndk(ndk_build: &str) {
    // let sdl2_libs: PathBuf = ["android", "app", "jni", "SDL", "libs"].iter().collect();
    let NDK_PROJECT_PATH: &str = ".";
    let APP_BUILD_SCRIPT: &str = "Android.mk";
    let APP_PLATFORM: &str = "android-31";

    // TODO: NDK_BUILD is a wrapper around make, use make instead
    // TODO: Work towards portability
    // See https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/reference/build-scripts.html
    Command::new(ndk_build)
            .args(&[format!("NDK_PROJECT_PATH={}", NDK_PROJECT_PATH),
                    format!("APP_BUILD_SCRIPT={}", APP_BUILD_SCRIPT),
                    format!("APP_PLATFORM={}", APP_PLATFORM)])
            .status().unwrap();
}

fn make_deps_dir(tag: &str, build_mode: &str) {
    // See https://doc.rust-lang.org/cargo/reference/environment-variables.html
    // let target_dir = env::var("CARGO_TARGET_DIR").expect("Cargo Target Directory Not Set");
    let target_dir = "target";
    let path: PathBuf = [target_dir, tag, build_mode, "deps"].iter().collect();

    fs::create_dir_all(path).unwrap();
}

fn copy_deps(tag: &str, build_mode: &str) {
    // let target_dir = env::var("CARGO_TARGET_DIR").expect("Cargo Target Directory Not Set");
    let target_dir = "target";
    let path: PathBuf = [target_dir, tag, build_mode, "deps"].iter().collect();

    fs::create_dir_all(path).unwrap();
}