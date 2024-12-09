use std::{io::Error, path::Path, sync::OnceLock};

use common::resources::{EmbeddedFile, EmbeddedFiles};

/// This is where resources are embedded if embedding is compiled in
static EMBEDDED_RESOURCES: OnceLock<EmbeddedFiles> = OnceLock::new();

/// This resource loader attempts to locate resources in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this binary
/// * Return an error
///
/// # Errors
///
/// May error if it cannot load the resource from the filesystem and from within the binary in that order
pub fn get_resource_bytes(path: &Path) -> Result<Vec<u8>, String> {
    if let Ok(file) = get_resource_file_from_filesystem(path) {
        return Ok(file);
    }

    get_embedded_resource_file(path)
}

/// This resource loader attempts to locate resources in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this binary
/// * Return an error
///
/// # Errors
///
/// May error if it cannot load the resource from the filesystem and from within the binary in that order
///
/// # Panics
///
/// May panic if cannot unwrap bytes as UTF-8 string
pub fn get_resource_string(path: &Path) -> Result<String, String> {
    let resource_bytes_option: Result<Vec<u8>, String> = get_resource_bytes(path);

    // Checks if an error and passes up
    resource_bytes_option.as_ref()?;

    Ok(String::from_utf8(resource_bytes_option.unwrap()).unwrap())
}

/// Attempts to retrieve a resource file from within the filesystem
///
/// TODO: Sanitize the File Path
fn get_resource_file_from_filesystem(path: &Path) -> Result<Vec<u8>, Error> {
    std::fs::read(path)
}

/// Attempts to retrieve a resource file from within the binary
fn get_embedded_resource_file(path: &Path) -> Result<Vec<u8>, String> {
    let embedded_files: &Vec<EmbeddedFile> = &EMBEDDED_RESOURCES.get().unwrap().inner;

    for file in embedded_files {
        if Path::new(&file.path).eq(path) {
            return Ok(file.contents.clone());
        }
    }

    Err(format!(
        "Asset Not Found - Unable To Load Embedded Asset: {path:?}"
    ))
}

/// Allows using embedded resources from this utility crate
pub fn store_embedded_resources(embedded_resources: EmbeddedFiles) {
    let _ = EMBEDDED_RESOURCES.set(embedded_resources);
}
