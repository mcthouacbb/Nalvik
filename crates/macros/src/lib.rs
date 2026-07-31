mod vertex_to_fragment;

use proc_macro::TokenStream;

#[proc_macro_derive(VertexToFragment)]
pub fn vertex_to_fragment_derive(input: TokenStream) -> TokenStream {
    vertex_to_fragment::vertex_to_fragment_derive_impl(input)
}
