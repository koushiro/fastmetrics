use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result};

use crate::utils::wrap_in_const;

pub fn expand_derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Only works for enums with at least one variant
    let data_enum = match &input.data {
        Data::Enum(data) => data,
        _ => {
            let error = "#[derive(EncodeLabelValue)] can only be used on enums";
            return Err(Error::new_spanned(name, error));
        },
    };
    // Ensure the enum has at least one variant
    if data_enum.variants.is_empty() {
        let error = "#[derive(EncodeLabelValue)] requires at least one variant in the enum";
        return Err(Error::new_spanned(name, error));
    }

    // Generate match arms for each variant
    let variant_arms = data_enum
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;

            // Check that this is a unit variant (no fields)
            match &variant.fields {
                Fields::Unit => {},
                _ => {
                    let error =
                        "#[derive(EncodeLabelValue)] can only be used for enums with unit variants";
                    return Err(Error::new_spanned(variant, error));
                },
            }

            // The string representation is the variant name
            let variant_str = variant_name.to_string();

            Ok(quote! {
                #name::#variant_name => encoder.encode_str_value(&#variant_str)?
            })
        })
        .collect::<Result<Vec<_>>>()?;

    // Generate the `EncodeLabelValue` trait implementation
    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::encoder::EncodeLabelValue for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelEncoder) -> ::fastmetrics::error::Result<()> {
                match self {
                    #(#variant_arms,)*
                }

                ::core::result::Result::Ok(())
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}
