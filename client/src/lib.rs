use winit::window::Icon;

#[macro_use]
extern crate tracing;

pub mod game_loop;

pub(crate) fn get_icon() -> Icon {
    // TODO: Implement proper asset finding and loading
    let image_bytes = include_bytes!("../assets/vanilla/texture/logo/logo.png");
    let image = image::load_from_memory(image_bytes)
        .expect("Could not get asset...").into_rgba8();
    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).unwrap()
}