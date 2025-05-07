use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, FieldsNamed, Result};

use crate::utils::wrap_in_const;

pub fn expand_derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Only works for structs with named fields
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            _ => {
                let error =
                    "#[derive(EncodeLabelSet)] can only be used for structs with named fields.";
                return Err(Error::new_spanned(name, error));
            },
        },
        _ => {
            let error = "#[derive(EncodeLabelSet)] can only be used for structs.";
            return Err(Error::new_spanned(name, error));
        },
    };

    // Process all fields
    let field_list = fields
        .into_iter()
        .map(|f| {
            let ident = f.ident.as_ref().expect("fields must be named");
            let ident_str = ident.to_string();
            quote! {
                encoder.encode(&(#ident_str, &self.#ident))?
            }
        })
        .collect::<Vec<_>>();

    let is_empty = field_list.is_empty();

    // Generate the `EncodeLabelSet` trait implementation
    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::encoder::EncodeLabelSet for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelSetEncoder) -> ::core::fmt::Result {
                use ::fastmetrics::encoder::EncodeLabel;

                #(#field_list;)*

                ::core::result::Result::Ok(())
            }

            #[inline]
            fn is_empty(&self) -> bool {
                #is_empty
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}
