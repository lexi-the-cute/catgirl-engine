//! Procedural Macros for the game engine

#![warn(missing_docs)]

use quote::ToTokens;
use syn::{spanned::Spanned, LitStr};

/// Embeds resources folder into the binary
///
/// # Panics
///
/// Panics if passed no tokens or tokens other than a literal or env!() macro
#[proc_macro]
pub fn generate_embedded_resources(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parses tokens into a rust expression
    let tokens_expr = syn::parse_macro_input!(tokens as syn::Expr);

    // Parse Resources Path
    let resources_path_result: Result<String, proc_macro::TokenStream> =
        parse_resource_macro(tokens_expr);
    if let Err(error_message) = resources_path_result {
        return error_message;
    }

    let resources_path: String = resources_path_result.unwrap();
    println!("Resources Path: {resources_path:?}");

    quote::quote! {
        pub(super) fn get_embedded_resources() -> std::string::String {
            // Ignore this for now
            #resources_path.to_string()
        }
    }
    .into()
}

/// Parses resource macros
fn parse_resource_macro(tokens: syn::Expr) -> Result<std::string::String, proc_macro::TokenStream> {
    match tokens {
        syn::Expr::Lit(path_lit) => {
            // let path = tokens as syn::LitStr;

            // Returns "\"path/to/resources\"" or 123
            // Ok(path_lit.into_token_stream().to_string())
            Ok(syn::parse2::<LitStr>(path_lit.to_token_stream())
                .unwrap()
                .value())
        }
        syn::Expr::Macro(path_macro) => {
            // Returns environment variable not found
            // Environment variable set in main crate build.rs
            Ok(parse_expr_macro(path_macro).unwrap())
        }
        _ => Err(syn::Error::new(
            tokens.span(),
            "Cannot parse embedded resource expression...",
        )
        .to_compile_error()
        .into()),
    }
}

/// Parses Expression Macros
fn parse_expr_macro(
    macro_token: syn::ExprMacro,
) -> Result<std::string::String, std::string::String> {
    let macro_segments: &syn::PathSegment = macro_token.mac.path.segments.first().unwrap();
    let macro_identifier: String = macro_segments.ident.to_string();

    if macro_identifier.eq("env") {
        let macro_tokens = macro_token.mac.tokens;
        let macro_string: std::string::String =
            syn::parse2::<LitStr>(macro_tokens).unwrap().value();
        let env_var: Result<String, std::env::VarError> = std::env::var(macro_string);

        if let Ok(environment_variable) = env_var {
            return Ok(environment_variable);
        }
    }

    Err("Could not parse expression macro...".to_string())
}
