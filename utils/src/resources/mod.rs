use serde::{Deserialize, Serialize};
use std::{path::Path, sync::OnceLock};

/// Embedded Files
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmbeddedFile {
    /// Relative File Path
    pub path: String,

    /// Contents of File
    pub contents: Vec<u8>,
}

/// Embedded Files
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EmbeddedFiles {
    /// Vector containing embedded files
    pub inner: Vec<EmbeddedFile>,
}

/// This is where resources are embedded if embedding is compiled in
// #[cfg(feature = "embed-resources")]
static EMBEDDED_RESOURCES: OnceLock<EmbeddedFiles> = OnceLock::new();

/// This resource loader attempts to locate resources in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this binary
/// * Return an error
pub fn get_resource_bytes(path: std::path::PathBuf) -> Option<Vec<u8>> {
    let embedded_files: &Vec<EmbeddedFile> = &EMBEDDED_RESOURCES.get().unwrap().inner;

    for file in embedded_files {
        if Path::new(&file.path).eq(&path) {
            return Some(file.contents.clone());
        }
    }

    None
}

/// This resource loader attempts to locate resources in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this binary
/// * Return an error
pub fn get_resource_string(path: std::path::PathBuf) -> Option<String> {
    let resource_bytes_option: Option<Vec<u8>> = get_resource_bytes(path);

    if resource_bytes_option.is_none() {
        return None;
    }

    Some(String::from_utf8(resource_bytes_option.unwrap()).unwrap())
}

/// Allows using embedded resources from this utility crate
// #[cfg(feature = "embed-resources")]
pub fn store_embedded_resources(embedded_resources: EmbeddedFiles) {
    let _ = EMBEDDED_RESOURCES.set(embedded_resources);
}
