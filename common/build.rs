//! Build script for crate

/// Main function
fn main() {
    // Debug environment
    // print_environment_vars();
}

/// Print all environment variables
#[allow(dead_code)]
fn print_environment_vars() {
    let vars: std::env::Vars = std::env::vars();

    println!("cargo:warning=Environment Variables:");
    for (key, var) in vars {
        if is_likely_secret(&key) {
            println!("cargo:warning=Env: {key}: {}", mask_string(&var));
        } else {
            println!("cargo:warning=Env: {key}: {var}");
        }
    }
}

/// Determines if string represents a secret
fn is_likely_secret(key: &str) -> bool {
    match key.to_lowercase() {
        // Very Likely
        s if s.contains("password") => true,
        s if s.contains("secret") => true,
        s if s.contains("token") => true,

        // Kinda Iffy
        s if s.contains("ssh") => true,
        s if s.contains("webhook") => true,
        s if s.contains("release_key") => true,
        s if s.contains("release_store") => true,

        // Iffy
        s if s.contains("account") => true,
        _ => false,
    }
}

/// Repeats a string an arbitrary number of times
fn repeat_string(repetitions: usize, value: &str) -> String {
    let mut buffer: Vec<&str> = Vec::new();

    for _ in 0..repetitions {
        buffer.push(value);
    }

    buffer.join("")
}

/// Masks a secret
fn mask_string(value: &str) -> String {
    let size: usize = value.chars().count();
    repeat_string(size, "*")
}
