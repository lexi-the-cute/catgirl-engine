//! Utilities for the game engine

#![warn(missing_docs)]

#[allow(unused_imports)]
#[macro_use]
extern crate tracing;

/// Module for macros
#[macro_use]
pub mod macros;

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
