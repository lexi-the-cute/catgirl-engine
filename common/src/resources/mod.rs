use serde::{Deserialize, Serialize};

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
