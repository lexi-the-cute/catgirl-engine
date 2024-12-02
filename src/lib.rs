//! Starting point for the game engine as a library

#![warn(missing_docs)]

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
