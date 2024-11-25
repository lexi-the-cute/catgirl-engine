//! Server side component of the game engine

#![warn(missing_docs)]

#[macro_use]
extern crate tracing;

/// Module for storing and using build data
pub mod build;

/// Handles server side game logic
pub mod game;
