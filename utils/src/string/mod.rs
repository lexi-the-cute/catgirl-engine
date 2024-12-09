/// Repeats a string an arbitrary number of times
fn repeat_string(repetitions: usize, value: &str) -> String {
    let mut buffer: Vec<&str> = Vec::new();

    for _ in 0..repetitions {
        buffer.push(value);
    }

    buffer.join("")
}

/// Masks a secret
pub(crate) fn mask_string(value: &str) -> String {
    let size: usize = value.chars().count();
    repeat_string(size, "*")
}

/// Determines if string represents a secret
pub(crate) fn is_likely_secret(key: &str) -> bool {
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
