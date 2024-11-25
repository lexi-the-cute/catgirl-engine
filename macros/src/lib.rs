//! Procedural Macros for the game engine

#![warn(missing_docs)]

use proc_macro::TokenStream;

/// Generates an asset loader function
///
/// The generated function will first attempt to in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this function
/// * Return an error
#[proc_macro]
pub fn generate_asset_loader(_tokens: TokenStream) -> TokenStream {
    _tokens;

    "".parse().unwrap()
}
