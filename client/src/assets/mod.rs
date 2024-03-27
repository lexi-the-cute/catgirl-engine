//! Handles assets loading

/// Load asset as binary
#[macro_export]
macro_rules! load_bytes {
    ($file:literal) => {{
        let external_assets_path: PathBuf = crate::game::get_assets_path().join($file);
        trace!("External Assets Path (Bytes): {:?}", external_assets_path);

        let embedded_bytes: Vec<u8> = std::fs::read(&external_assets_path).unwrap_or_else(|_| {
            trace!(
                "Asset Not Found - Loading Embedded Asset (Bytes): {:?}",
                external_assets_path
            );
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file)).to_vec()
        });

        embedded_bytes
    }};
}

/// Load asset as string
#[macro_export]
macro_rules! load_string {
    ($file:literal) => {{
        let external_assets_path: PathBuf = crate::game::get_assets_path().join($file);
        trace!("External Assets Path (String): {:?}", external_assets_path);

        let embedded_string: String = std::fs::read_to_string(&external_assets_path)
            .unwrap_or_else(|_| {
                trace!(
                    "Asset Not Found - Loading Embedded Asset (String): {:?}",
                    external_assets_path
                );
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file)).to_string()
            });

        embedded_string
    }};
}
