use build_info::GitInfo;

// Generate build_info() function at compile time
build_info::build_info!(
    /// Build info for crate
    pub fn build_info
);

/// Retrieves the commit hash of the repo when this was built
#[must_use]
pub fn get_version_control_build_info() -> Option<GitInfo> {
    build_info().version_control.as_ref()?.git().cloned()
}
