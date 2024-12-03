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
    use syn::spanned::Spanned;

    // Parses tokens into a rust expression
    let tokens_expr_result: Result<Expr, syn::Error> = syn::parse::<Expr>(tokens);
    if let Err(error) = tokens_expr_result {
        return Err(error.to_compile_error().into());
    }

    // Match against the two types of tokens we care about and parse
    let tokens_expr: Expr = tokens_expr_result.unwrap();
    match tokens_expr {
        syn::Expr::Lit(path_lit) => Ok(syn::parse2::<syn::LitStr>(
            quote::ToTokens::to_token_stream(&path_lit),
        )
        .unwrap()
        .value()),
        syn::Expr::Macro(path_macro) => Ok(parse_expr_macro(path_macro).unwrap()),
        _ => Err(syn::Error::new(
            tokens_expr.span(),
            "Cannot parse embedded resource expression...",
        )
        .to_compile_error()
        .into()),
    }
}

/// Parses Expression Macros
fn parse_expr_macro(macro_token: syn::ExprMacro) -> Result<String, String> {
    let macro_segments: &syn::PathSegment = macro_token.mac.path.segments.first().unwrap();
    let macro_identifier: String = macro_segments.ident.to_string();

    if macro_identifier.eq("env") {
        let macro_tokens = macro_token.mac.tokens;
        let macro_string: String = syn::parse2::<LitStr>(macro_tokens).unwrap().value();
        let env_var: Result<String, std::env::VarError> = std::env::var(macro_string);

        if let Ok(environment_variable) = env_var {
            return Ok(environment_variable);
        }
    }

    Err("Could not parse expression macro...".to_string())
}
