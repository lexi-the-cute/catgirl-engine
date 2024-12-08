//! Utilities for the game engine
#![warn(missing_docs)]
#![doc(
    html_favicon_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_logo_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_playground_url = "https://play.rust-lang.org"
)]

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

/// Module for macros
#[macro_use]
mod macros;

/// Module for handling translations
pub mod i18n;

/// Module for command line arguments
pub mod args;

/// Module for storing and using resources
pub mod resources;

/// Module for storing and using build data
pub mod build;

/// Module for reading the environment variables
pub mod environment;

/// Module for handling the exit state of the engine
pub mod exit;

/// Module for handling strings
pub mod string;
