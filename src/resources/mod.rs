// Where resources are embedded in the binary
// #[cfg(feature = "embed-resources")]
// env!("ENGINE_RESOURCES_PATH")
macros::generate_embedded_resources!(env!("ENGINE_RESOURCES_PATH"));
