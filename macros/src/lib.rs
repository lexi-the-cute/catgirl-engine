//! Procedural Macros for the game engine
#![warn(missing_docs)]
#![doc(
    html_favicon_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_logo_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_playground_url = "https://play.rust-lang.org"
)]

use common::resources::{EmbeddedFile, EmbeddedFiles};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use proc_macro::TokenStream;
use syn::{Expr, LitStr};

/// Embeds resources folder into the binary
///
/// # Panics
///
/// Panics if passed no tokens or tokens other than a literal or env!() macro
#[proc_macro]
pub fn generate_embedded_resources(tokens: TokenStream) -> TokenStream {
    // Parse Resources Path
    let resources_path_result: Result<String, TokenStream> = parse_resource_macro(tokens);
    if let Err(error_message) = resources_path_result {
        return error_message;
    }

    let files: EmbeddedFiles = get_files(&PathBuf::from(&resources_path_result.unwrap()));
    let files_json: String = serde_json::to_string(&files).unwrap();

    quote::quote! {
        pub(super) fn get_embedded_resources() -> common::resources::EmbeddedFiles {
            let files_json: String = #files_json.to_string();
            serde_json::from_str::<common::resources::EmbeddedFiles>(&files_json).unwrap()
        }
    }
    .into()
}

/// Determines if a path should be embedded
#[rustfmt::skip]
fn should_embed(path: &Path) -> bool {
    let resources_path: PathBuf = PathBuf::from("resources");

    [
        // Locales should always be embedded
        resources_path.join("locales"),

        // Only embed if the build target does not support filesystems
        #[cfg(feature = "embed-assets")]
        resources_path.join("assets"),

        #[cfg(all(feature = "embed-assets", target_os = "linux"))]
        resources_path.join("linux"),

        #[cfg(all(feature = "embed-assets", target_os = "windows"))]
        resources_path.join("windows"),

        #[cfg(all(feature = "embed-assets", target_os = "macos"))]
        resources_path.join("osx"),

        #[cfg(all(feature = "embed-assets", target_os = "android"))]
        resources_path.join("android"),

        #[cfg(all(feature = "embed-assets", target_os = "ios"))]
        resources_path.join("ios"),

        #[cfg(all(feature = "embed-assets", target_family = "wasm"))]
        resources_path.join("wasm"),
    ]
    .into_iter()
    .any(|embed_path| path.starts_with(embed_path))
}

/// Recursively retrieve within the specified directory
fn get_files(resources_path: &Path) -> EmbeddedFiles {
    let mut dirs: VecDeque<std::fs::ReadDir> =
        VecDeque::from([std::fs::read_dir(resources_path).unwrap()]);
    let mut files: EmbeddedFiles = EmbeddedFiles { inner: Vec::new() };

    while !dirs.is_empty() {
        let dir: std::fs::ReadDir = VecDeque::pop_front(&mut dirs).unwrap();

        for dir_entry in dir {
            let full_path: PathBuf = dir_entry.as_ref().unwrap().path();
            let path: PathBuf = shorten_file_paths(resources_path, &full_path);

            if full_path.is_dir() {
                dirs.push_back(std::fs::read_dir(&full_path).unwrap());
            } else if should_embed(&path) {
                // println!("FP: {:?}; SFP: {:?}", &full_path, &path);
                files.inner.push(EmbeddedFile {
                    path: path.to_str().unwrap().to_string(),
                    contents: std::fs::read(path).unwrap(),
                });
            }
        }
    }

    files
}

/// Shorten the path to only the part after the tail end
fn shorten_file_paths(resources_path: &Path, file_path: &Path) -> PathBuf {
    let path_components: std::path::Components<'_> = file_path.components();

    let mut shortened_file_path: PathBuf = PathBuf::new();
    let mut temp_base_path: PathBuf = PathBuf::new();
    let mut found_root_path: bool = false;
    for component in path_components {
        temp_base_path.push(component);

        if !found_root_path && temp_base_path.eq(resources_path) {
            temp_base_path.clear();
            temp_base_path.push(PathBuf::from(resources_path.file_name().unwrap()));

            found_root_path = true;
        }

        if found_root_path {
            shortened_file_path.clone_from(&temp_base_path);
        }
    }

    shortened_file_path
}

/// Parses resource macros
fn parse_resource_macro(tokens: TokenStream) -> Result<String, TokenStream> {
    // Parses tokens into a rust expression
    let token_expr_result: Result<Expr, syn::Error> = syn::parse::<Expr>(tokens);
    if let Err(error) = token_expr_result {
        return Err(error.to_compile_error().into());
    }

    // Match against the two types of tokens we care about and parse
    let token_expr: Expr = token_expr_result.unwrap();
    match token_expr {
        // Parses for string literal ("/path/to/resources")
        syn::Expr::Lit(path_lit) => Ok(parse_string_literal(path_lit)),
        syn::Expr::Macro(path_macro) => parse_expr_macro(path_macro),
        _ => Err(create_error(
            token_expr,
            "Cannot parse embedded resource expression...",
        )),
    }
}

/// Parses String Literals
fn parse_string_literal(string_literal: syn::ExprLit) -> String {
    syn::parse2::<syn::LitStr>(quote::ToTokens::to_token_stream(&string_literal))
        .unwrap()
        .value()
}

/// Parses Expression Macros
fn parse_expr_macro(macro_token: syn::ExprMacro) -> Result<String, TokenStream> {
    let macro_segments: &syn::PathSegment = macro_token.mac.path.segments.first().unwrap();
    let macro_identifier: String = macro_segments.ident.to_string();

    if macro_identifier.eq("env") {
        let macro_tokens = macro_token.mac.tokens.clone();
        let macro_string: String = syn::parse2::<LitStr>(macro_tokens).unwrap().value();
        let env_var: Result<String, std::env::VarError> = std::env::var(macro_string);

        if let Ok(environment_variable) = env_var {
            return Ok(environment_variable);
        }
    }

    Err(create_error(
        macro_token.into(),
        "Could not parse expression macro...",
    ))
}

/// Create's an error as a `TokenStream` to unwind to compiler
fn create_error(token_expr: Expr, message: &str) -> TokenStream {
    use syn::spanned::Spanned;

    syn::Error::new(token_expr.span(), message)
        .to_compile_error()
        .into()
}
