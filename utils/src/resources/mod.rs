use std::sync::OnceLock;

/// This is where resources are embedded if embedding is compiled in
// #[cfg(feature = "embed-resources")]
static EMBEDDED_RESOURCES: OnceLock<String> = OnceLock::new();

/// This resource loader attempts to locate resources in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this binary
/// * Return an error
pub fn get_resource(_path: std::path::PathBuf) {
    todo!()
}

/// Allows using embedded resources from this utility crate
// #[cfg(feature = "embed-resources")]
pub fn store_embedded_resources(embedded_resources: String) {
    let _ = EMBEDDED_RESOURCES.set(embedded_resources);
}
