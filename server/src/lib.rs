//! Server side component of the game engine
#![warn(missing_docs)]
#![doc(
    html_favicon_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_logo_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_playground_url = "https://play.rust-lang.org"
)]

#[macro_use]
extern crate tracing;

/// Module for storing and using build data
pub mod build;

/// Handles server side game logic
pub mod game;
