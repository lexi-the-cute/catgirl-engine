use std::env;

fn main() {
    let target_family: String = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();

    if !target_family.contains("wasm") {
        println!("cargo:warning=wasm_threads is only meant to be used with the wasm family");
    }
}
