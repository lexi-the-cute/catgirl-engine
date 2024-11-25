//! Build script for crate

/// Main function
fn main() {
    // Debug environment
    print_environment_vars();
}

/// Print all environment variables
#[allow(dead_code)]
fn print_environment_vars() {
    let vars: std::env::Vars = std::env::vars();

    for (key, var) in vars {
        println!("cargo:warning=EV: {key}: {var}");
    }
}
