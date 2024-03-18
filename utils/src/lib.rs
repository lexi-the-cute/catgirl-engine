//! Utilities for the catgirl-engine crate

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::pedantic)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::items_after_statements)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::doc_markdown)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::len_zero)]
#![warn(clippy::uninlined_format_args)]

/// Module for command line arguments
pub mod args;

/// Handles setup
pub mod setup;
