// macros::generate_asset_loader!(env!("ENGINE_ASSETS_PATH"));

macro_rules! store_assets {
    // Traverses the asset tree and stores all files
    () => {{
        // for file in fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/args/"))? {
        //     // include_str!(file.path());

        //     warn!("File: {}", file.path());
        // }
    }};
}

/// Allows embedding the assets into the binary at build time
pub fn store_assets() {
    // let directory_handle: fs::ReadDir = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/args/"))?;

    // for file in directory_handle {
    //     // include_str!(file.path());

    //     warn!("File: {}", file.path());
    // }

    store_assets!();
}
