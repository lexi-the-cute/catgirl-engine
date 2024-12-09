use serde::{Deserialize, Serialize};

/// Embedded Files
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Default, Hash, Eq, Ord)]
pub struct EmbeddedFile {
    /// Relative File Path
    pub path: String,

    /// Contents of File
    pub contents: Vec<u8>,
}

/// Embedded Files
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Default, Hash, Eq, Ord)]
pub struct EmbeddedFiles {
    /// Vector containing embedded files
    pub inner: Vec<EmbeddedFile>,
}
