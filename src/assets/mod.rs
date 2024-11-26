// Where assets are embedded in the binary
// #[cfg(feature = "embed-assets")]
// env!("ENGINE_ASSETS_PATH")
macros::generate_embedded_assets!("path/to/assets");
