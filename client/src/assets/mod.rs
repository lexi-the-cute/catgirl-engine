//! Handles assets loading

/// Load asset as binary
#[macro_export]
macro_rules! load_bytes {
    ($file:literal) => {{
        let external_assets_path: PathBuf = $crate::game::get_assets_path().join($file);
        trace!("External Assets Path (Bytes): {:?}", external_assets_path);

        let embedded_bytes: Result<Vec<u8>, String> = {
            // Attempt to read asset externally first
            let file_result: Result<Vec<u8>, std::io::Error> = std::fs::read(&external_assets_path);
            if let Ok(file) = file_result {
                Ok(file)
            } else if cfg!(feature = "embed-assets") {
                // Attempts to read asset from within this binary
                trace!(
                    "Asset Not Found - Loading Embedded Asset (Bytes): {:?}",
                    external_assets_path
                );

                Ok(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file)).to_vec())
            } else {
                trace!(
                    "Asset Not Found - Unable To Load Asset (Bytes): {:?}",
                    external_assets_path
                );

                Err(format!(
                    "Asset Not Found - Unable To Load Asset (Bytes): {:?}",
                    external_assets_path
                ))
            }
        };

        embedded_bytes
    }};
}

/// Load asset as string
#[macro_export]
macro_rules! load_string {
    ($file:literal) => {{
        let external_assets_path: PathBuf = $crate::game::get_assets_path().join($file);
        trace!("External Assets Path (String): {:?}", external_assets_path);

        let embedded_string: Result<String, String> = {
            // Attempt to read asset externally first
            let file_result: Result<String, std::io::Error> =
                std::fs::read_to_string(&external_assets_path);
            if let Ok(file) = file_result {
                Ok(file)
            } else if cfg!(feature = "embed-assets") {
                // Attempts to read asset from within this binary
                trace!(
                    "Asset Not Found - Loading Embedded Asset (String): {:?}",
                    external_assets_path
                );

                Ok(
                    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file))
                        .to_string(),
                )
            } else {
                trace!(
                    "Asset Not Found - Unable To Load Asset (String): {:?}",
                    external_assets_path
                );

                return Err(format!(
                    "Asset Not Found - Unable To Load Asset (String): {:?}",
                    external_assets_path
                ));
            }
        };

        embedded_string
    }};
}
