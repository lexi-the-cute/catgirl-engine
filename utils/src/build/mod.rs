use build_info::GitInfo;
use std::sync::OnceLock;

static ENGINE_NAME: OnceLock<String> = OnceLock::new();

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

/// Set's the engine's name for both license compliance and naming
pub fn set_engine_name(engine_name: String) {
    let _ = ENGINE_NAME.set(engine_name);
}

/// Get's the engine's name for both license compliance and naming
pub fn get_engine_name() -> String {
    ENGINE_NAME
        .get()
        .unwrap_or(&"game-engine".to_string())
        .to_string()
}
