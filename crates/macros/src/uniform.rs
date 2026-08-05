use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::parse_macro_input;

pub fn uniform_derive_impl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let crate_name = match crate_name("rasterizer") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => quote!(::#name),
        Err(_) => {
            return quote! {
                compile_error!("Could not find dependency rasterizer.")
            }
            .into();
        }
    };

    let trait_path = quote! {
        #crate_name::render::pipeline::vertex_to_fragment::VertexToFragment
    };
}
