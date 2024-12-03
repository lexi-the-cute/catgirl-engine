//! Procedural Macros for the game engine
#![warn(missing_docs)]
#![doc(
    html_favicon_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_logo_url = "https://engine.catgirl.land/resources/assets/vanilla/texture/logo/logo.svg",
    html_playground_url = "https://play.rust-lang.org"
)]

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

    let resources_path: String = resources_path_result.unwrap();
    // println!("Resources Path: {resources_path:?}");

    quote::quote! {
        pub(super) fn get_embedded_resources() -> String {
            // Ignore this for now
            #resources_path.to_string()
        }
    }
    .into()
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

// Parses String Literals
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

/// Create's an error as a TokenStream to unwind to compiler
fn create_error(token_expr: Expr, message: &str) -> TokenStream {
    use syn::spanned::Spanned;

    syn::Error::new(token_expr.span(), message)
        .to_compile_error()
        .into()
}
