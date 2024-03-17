// Generate build_info() function at compile time
#[cfg(feature = "build-info")]
build_info::build_info!(
    /// Build info for crate
    pub fn build_info
);
