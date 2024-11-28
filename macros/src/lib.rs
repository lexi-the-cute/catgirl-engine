//! Procedural Macros for the game engine

#![warn(missing_docs)]

use quote::ToTokens;
use syn::{spanned::Spanned, LitStr};

// struct ResourceMacroInput {
//     expr: T
// }

// impl syn::parse::Parse for ResourceMacroInput {
//     fn parse(input: syn::parse::ParseStream) -> Result<Self> {
//         if input.peek(syn::Token![litexpr]) {
//             Ok(ResourceMacroInput {
//                 expr: input.parse()?,
//             })
//         } else {
//             Err(input.error("expected some kind of loop"))
//         }
//     }
// }

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
        fn get_embedded_resources() -> std::string::String {
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
            Ok(syn::parse::<LitStr>(path_lit.to_token_stream().into())
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
        let macro_string = syn::parse::<LitStr>(macro_tokens.into()).unwrap().value();
        let env_var: Result<String, std::env::VarError> = std::env::var(macro_string);

        if let Ok(environment_variable) = env_var {
            return Ok(environment_variable);
        }
    }

    Err("Could not parse expression macro...".to_string())
}

/// Generates `macros_build_info()`
///
/// Waiting for feature request to be implemented:
///
/// <https://github.com/danielschemmel/build-info/issues/25>
#[proc_macro]
pub fn macros_build_info(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let build_info: &str = env!("BUILD_INFO");
    quote::quote! {
        /// Build info for crate
        fn macros_build_info() -> build_info::BuildInfo {
            build_info::proc::custom_build_info(
                #build_info
            );
        }
    }
    .into()
}
