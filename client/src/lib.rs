//! Client side component of the catgirl-engine crate

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::pedantic)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::len_zero)]
#![warn(clippy::uninlined_format_args)]

use std::{fs, path::PathBuf};
use winit::window::Icon;

#[macro_use]
extern crate tracing;

/// Handles the client side game logic
pub mod game;

/// Handles the client window
pub mod window;

/// Handles the rendering code
pub mod render;

/// Handles setup
pub mod setup;

/// Retrieve the engine's icon
///
/// This does not work on Wayland, use the .desktop shortcut
///
/// # Panics
///
/// This may fail to load the file from the byte array as an image
// TODO (BIND): Implement `#[wasm_bindgen]` and `extern "C"`
#[must_use]
pub fn get_icon() -> Icon {
    let assets_path: PathBuf = crate::game::get_assets_path();
    let logo_path: PathBuf = assets_path.join("vanilla/texture/logo/logo.png");

    let image_vec_result: Result<Vec<u8>, std::io::Error> = fs::read(logo_path);
    let image_bytes: &[u8] = image_vec_result.as_deref().unwrap_or({
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/vanilla/texture/logo/logo.png"
        ))
    });

    let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::load_from_memory(image_bytes)
        .expect("Could not get asset from memory...")
        .into_rgba8();
    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).unwrap()
}
