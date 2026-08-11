use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

pub fn vertex_to_fragment_derive_impl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let crate_name = match crate_name("nalvik") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let name = syn::Ident::new(name.as_str(), proc_macro2::Span::call_site());
            quote!(::#name)
        }
        Err(_) => {
            return quote! {
                compile_error!("Could not find dependency nalvik.")
            }
            .into();
        }
    };

    let trait_path = quote! {
        #crate_name::VertexToFragment
    };

    match ast.data {
        Data::Struct(structure) => {
            let name = ast.ident;
            match structure.fields {
                Fields::Named(fields) => {
                    let field_names = fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<&Ident>>();
                    quote! {
                        impl #trait_path for #name {
                            fn scale_w(&mut self, scale: f32) {
                                #(self.#field_names *= scale;)*
                            }

                            fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
                                Self {
                                    #(#field_names: a.#field_names * (1.0 - t) + b.#field_names * t),*
                                }
                            }

                            fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
                                Self {
                                    #(#field_names: a.#field_names * barycentric.x + b.#field_names * barycentric.y + c.#field_names * barycentric.z),*
                                }
                            }
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_names = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| quote!(#idx))
                        .collect::<Vec<proc_macro2::TokenStream>>();
                    quote! {
                        impl #trait_path for #name {
                            fn scale_w(&mut self, scale: f32) {
                                #(self.#field_names *= scale;)*
                            }

                            fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
                                Self {
                                    #(#field_names: a.#field_names * (1.0 - t) + b.#field_names * t),*
                                }
                            }

                            fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
                                Self {
                                    #(#field_names: a.#field_names * barycentric.x + b.#field_names * barycentric.y + c.#field_names * barycentric.z),*
                                }
                            }
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        impl #trait_path for #name {
                            fn scale_w(&mut self, scale: f32) {}
                            fn interpolate2(a: &Self, b: &Self, t: f32) -> Self {
                                Self
                            }
                            fn interpolate3(a: &Self, b: &Self, c: &Self, barycentric: Vector3<f32>) -> Self {
                                Self
                            }
                        }
                    }
                }
            }
        }
        Data::Enum(syn_enum) => quote_spanned! {
            syn_enum.enum_token.span =>
            compile_error!("VertexToFragment cannot be derived on enums")
        },
        Data::Union(syn_union) => quote_spanned! {
            syn_union.union_token.span =>
            compile_error!("VertexToFragment cannot be derived on unions")
        },
    }
    .into()
}
