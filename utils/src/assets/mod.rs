/// Locate binary file for loading
#[macro_export]
macro_rules! bytes {
    ($file:expr $(,)?) => {{
        let embedded_bytes: &[u8] = include_bytes!($file);

        embedded_bytes
    }};
}

/// Locate string file for loading
#[macro_export]
macro_rules! string {
    ($file:expr $(,)?) => {{
        let embedded_string: String = include_str!($file).to_string();

        embedded_string
    }};
}
