//! Server side component of the catgirl-engine crate

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Handles server side game logic
pub mod game;

/// Handles setup
pub mod setup;
