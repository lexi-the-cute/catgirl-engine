use build_info::BuildInfo;

// Generate build_info() function at compile time
build_info::build_info!(fn build_info);

/// Build info for crate
pub fn build_info_pub() -> &'static BuildInfo {
    build_info()
}
