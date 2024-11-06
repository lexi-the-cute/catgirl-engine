//! Client side component of the catgirl-engine crate

#![warn(missing_docs)]

use std::{env, fs, path::PathBuf};
use winit::window::Icon;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[macro_use]
extern crate tracing;

// #[macro_use]
// extern crate utils;

/// Handles the client side game logic
pub mod game;

/// Handles the client window
pub mod window;

/// Handles the rendering code
pub mod render;

/// Handles setup
pub mod setup;

/// Module for handling assets
pub mod assets;

/// Retrieve the engine's icon as raw bytes
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn get_icon_bytes() -> Vec<u8> {
    load_bytes!("vanilla/texture/logo/logo-1024x1024-color.png")
}

/// Retrieve the engine's icon
///
/// This does not work on Wayland, use the .desktop shortcut
///
/// # Panics
///
/// This may fail to load the file from the byte array as an image
#[must_use]
pub fn get_icon() -> Icon {
    let image_bytes: Vec<u8> = get_icon_bytes();

    let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::load_from_memory(&image_bytes)
        .expect("Could not get asset from memory...")
        .into_rgba8();
    let (width, height) = image.dimensions();

    Icon::from_rgba(image.into_raw(), width, height).unwrap()
}

/// Install Linux desktop files
///
/// # Panics
///
/// May panic if cannot unwrap executable path
///
/// # Errors
///
/// May error if home directory cannot be found
pub fn install_desktop_files() -> Result<(), String> {
    let mut desktop_file_contents: String = load_string!("resources/catgirl-engine.desktop");

    // Get path of executable
    let executable_path: String =
        if let Some(app_image_path) = utils::get_environment_var("APPIMAGE") {
            app_image_path
        } else {
            env::current_exe()
                .expect("Could not get own path when installing desktop file...")
                .to_str()
                .unwrap()
                .to_string()
        };

    desktop_file_contents =
        desktop_file_contents.replace("${engine_path}", executable_path.as_str());

    if let Some(home) = utils::get_environment_var("HOME") {
        // User Application Directories
        let applications_directory: String = format!("{home}/.local/share/applications");
        let icons_directory: String = format!("{home}/.local/share/icons");

        // Install Paths
        let desktop_path: PathBuf =
            PathBuf::from(&applications_directory).join("catgirl-engine.desktop");
        let icon_path: PathBuf = PathBuf::from(&icons_directory).join("catgirl-engine.png");

        // Create folders if they don't exist
        let _ = fs::create_dir_all(&applications_directory);
        let _ = fs::create_dir_all(&icons_directory);

        // Remove old files if any
        let _ = fs::remove_file(&desktop_path);
        let _ = fs::remove_file(&icon_path);

        let _ = fs::write(desktop_path, desktop_file_contents);
        let _ = fs::write(icon_path, get_icon_bytes());

        return Ok(());
    }

    Err("Failed to find home directory".to_string())
}

/// Install Linux desktop files
///
/// # Panics
///
/// May panic if cannot unwrap executable path
///
/// # Errors
///
/// May error if home directory cannot be found
pub fn uninstall_desktop_files() -> Result<(), String> {
    if let Some(home) = utils::get_environment_var("HOME") {
        // User Application Directories
        let applications_directory: String = format!("{home}/.local/share/applications");
        let icons_directory: String = format!("{home}/.local/share/icons");

        // Install Paths
        let desktop_path: PathBuf =
            PathBuf::from(&applications_directory).join("catgirl-engine.desktop");
        let icon_path: PathBuf = PathBuf::from(&icons_directory).join("catgirl-engine.png");

        // Remove old files if any
        let _ = fs::remove_file(desktop_path);
        let _ = fs::remove_file(icon_path);

        return Ok(());
    }

    Err("Failed to find home directory".to_string())
}
