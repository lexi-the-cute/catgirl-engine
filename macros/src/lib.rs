//! Procedural Macros for the game engine

#![warn(missing_docs)]

use quote::ToTokens;
use syn::spanned::Spanned;

/// Embeds assets folder into the binary
#[proc_macro]
pub fn generate_embedded_assets(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if tokens.is_empty() {}

    let path_expr: syn::Expr = syn::parse_macro_input!(tokens as syn::Expr);

    let mut assets_path_option: Option<String> = None;
    match path_expr {
        syn::Expr::Lit(path_lit) => {
            // Returns "\"path/to/assets\"" or 123
            assets_path_option = Some(path_lit.into_token_stream().to_string());
        }
        syn::Expr::Macro(path_macro) => {
            // Returns environment variable not found
            // Environment variable set in main crate build.rs
            assets_path_option = Some(parse_expr_macro(path_macro).unwrap())
        }
        _ => {
            return syn::Error::new(
                path_expr.span(),
                "Cannot parse embedded asset expression...",
            )
            .to_compile_error()
            .into();
        }
    };

    let assets_path: String = assets_path_option.unwrap();
    println!("cargo:warning=Assets Path: {:?}", assets_path);

    quote::quote! {
        pub(crate) fn get_embedded_assets() -> std::string::String {
            // Ignore this for now
            #assets_path.to_string()
        }
    }
    .into()
}

fn parse_expr_macro(
    macro_token: syn::ExprMacro,
) -> Result<std::string::String, std::string::String> {
    let macro_segments: &syn::PathSegment = &macro_token.mac.path.segments.first().unwrap();
    let macro_identifier: String = macro_segments.ident.to_string();

    if macro_identifier.eq("env") {
        let macro_expr = &macro_token.mac.tokens;
        let macro_tokens: String = macro_expr.to_string();

        let env_var: Result<String, std::env::VarError> = std::env::var(macro_tokens.as_str());

        if let Ok(environment_variable) = env_var {
            return Ok(environment_variable);
        } else {
            return Err("Could not read environment variable...".to_string());
        }
    }

    Err("Could not parse expression macro...".to_string())
}

/// Generates macros_build_info()
///
/// Waiting for feature request to be implemented:
/// * https://github.com/danielschemmel/build-info/issues/25
#[proc_macro]
pub fn macros_build_info(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let build_info: &str = env!("BUILD_INFO");
    quote::quote! {
        /// Build info for crate
        pub fn macros_build_info() -> build_info::BuildInfo {
            build_info::proc::custom_build_info(
                #build_info
            );
        }
    }
    .into()
}
