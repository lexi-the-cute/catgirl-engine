use std::sync::OnceLock;

static EMBEDDED_ASSETS: OnceLock<String> = OnceLock::new();

/// This asset loader attempts to locate assets in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this function
/// * Return an error
pub fn get_asset(path: std::path::PathBuf) {
    // error!("{}", #assets_path);
}

/// Allows using embedded assets from this utility crate
// #[cfg(feature = "embed-assets")]
pub fn store_embedded_assets(embedded_assets: String) {
    let _ = EMBEDDED_ASSETS.set(embedded_assets);
}
