//! Client side component of the catgirl-engine crate

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use winit::window::Icon;

#[macro_use]
extern crate tracing;

/// Handles the client side game logic
pub mod game;

/// Handles the client window
pub mod window;

/// Retrieve the icon stored in the binary at build time
pub fn get_icon() -> Icon {
    // TODO: Implement proper asset finding and loading
    let image_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/vanilla/texture/logo/logo.png"
    ));

    // let image_bytes = include_bytes!("../assets/vanilla/texture/logo/logo.png");
    let image = image::load_from_memory(image_bytes)
        .expect("Could not get asset...")
        .into_rgba8();
    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).unwrap()
}
