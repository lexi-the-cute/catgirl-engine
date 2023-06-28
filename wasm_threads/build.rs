use std::env::{self, Vars};

fn main() {
    // print_environment_vars();

    let target_family: String = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let target: String = env::var("TARGET").unwrap();

    if !target_family.contains("wasm") {
        println!("cargo:warning=wasm_threads is only meant to be used with the wasm family");
    }

    if !target.contains("wasm32-unknown-unknown") && !target.contains("wasm64-unknown-unknown") {
        println!("cargo:warning=While wasm_threads is meant for the wasm family, it currently only supports wasm32-unknown-unknown and wasm64-unknown-unknown");
    }
}

#[allow(dead_code)]
fn print_environment_vars() {
    let vars: Vars = std::env::vars();

    for (key, var) in vars {
        println!("cargo:warning=EV: {key}: {var}");
    }
}