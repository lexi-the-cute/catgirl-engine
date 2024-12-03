//! Starting point for the game engine as a library
#![warn(missing_docs)]
#![doc(
    html_favicon_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_logo_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_playground_url = "https://play.rust-lang.org"
)]

#[macro_use]
extern crate tracing;

/// Implementation specific starting points
mod start;

/// Prepare the game engine for running
mod setup;

/// Module for storing and using build data
mod build;

/// Module for storing resources
mod resources;
