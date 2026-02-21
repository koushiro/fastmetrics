use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result};

use crate::utils::wrap_in_const;

pub fn expand_derive(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Only works for enums with at least one variant
    let data_enum = match &input.data {
        Data::Enum(data) => data,
        _ => {
            let error = "#[derive(StateSetValue)] can only be used for enums";
            return Err(Error::new_spanned(name, error));
        },
    };
    // Ensure the enum has at least one variant
    if data_enum.variants.is_empty() {
        let error = "#[derive(StateSetValue)] requires at least one variant in the enum";
        return Err(Error::new_spanned(name, error));
    }

    // Process all variants, ensuring they are unit variants
    let variants = data_enum
        .variants
        .iter()
        .map(|variant| {
            // Check if this is a unit variant (no fields)
            match variant.fields {
                Fields::Unit => {},
                _ => {
                    let error =
                        "#[derive(StateSetValue)] can only be used for enums with unit variants";
                    return Err(Error::new_spanned(variant, error));
                },
            }

            let variant_name = &variant.ident;
            let variant_str = variant_name.to_string();

            Ok((
                quote! { #name::#variant_name }, // For variants() method
                quote! { #name::#variant_name => #variant_str }, // For as_str() method
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    // Unzip the variants into separate vectors
    let (variant_list, variant_arms): (Vec<_>, Vec<_>) = variants.into_iter().unzip();

    // Generate the `StateSetValue` trait implementation
    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::metrics::state_set::StateSetValue for #name #ty_generics #where_clause {
            fn variants() -> &'static [Self] {
                static VARIANTS: &[#name] = &[
                    #(#variant_list,)*
                ];
                VARIANTS
            }

            fn as_str(&self) -> &'static str {
                match self {
                    #(#variant_arms,)*
                }
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}
