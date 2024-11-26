//! Procedural Macros for the game engine

#![warn(missing_docs)]

/// Generates an asset loader function
///
/// The generated function will first attempt to in this order:
/// * Locate and read a file from the filesystem
/// * Locate and read a file from within this function
/// * Return an error
#[proc_macro]
pub fn generate_asset_loader(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // if tokens.is_empty() {}

    println!("cargo:warning=Assets: {:?}", tokens);

    quote::quote! {
        ///
        pub fn get
    }
    .into()
}

/// ...
#[proc_macro]
pub fn test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path_lit: syn::LitStr = syn::parse(input).unwrap();
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let joined_path = AsRef::<std::path::Path>::as_ref(&root_dir).join(path_lit.value());
    let joined_path = joined_path.to_str().unwrap();

    quote::quote! { #joined_path }.into()
}

/// Generates `macros_build_info`
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
