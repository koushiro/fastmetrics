use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, FieldsNamed, Result};

use crate::{label_attributes::LabelAttributes, utils::wrap_in_const};

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

    // Process all fields with #[label(...)] attributes
    let parsed_fields = fields
        .iter()
        .map(|field| Ok((field, LabelAttributes::parse(field)?)))
        .collect::<Result<Vec<_>>>()?;

    let encode_stmts = parsed_fields
        .iter()
        .map(|(field, attrs)| {
            let ident = field.ident.as_ref().expect("fields must be named");

            // #[label(skip)] -> no encoding for this field
            if attrs.label.skip {
                return Ok(quote! { /* skip */ });
            }

            // #[label(flatten)] -> encode nested label set
            if attrs.label.flatten {
                return Ok(quote! {
                    ::fastmetrics::encoder::EncodeLabelSet::encode(&self.#ident, encoder)?
                });
            }

            // Determine the label name: rename override or field ident
            let field_name_tokens = if let Some(rename) = &attrs.label.rename {
                rename.to_token_stream()
            } else {
                let ident_str = ident.to_string();
                quote!(#ident_str)
            };

            Ok(quote! {
                encoder.encode(&(#field_name_tokens, &self.#ident))?
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let is_empty_exprs = parsed_fields
        .iter()
        .map(|(field, attrs)| {
            let ident = field.ident.as_ref().expect("fields must be named");

            if attrs.label.skip {
                // Skipped field contributes nothing
                Ok(quote! { true })
            } else if attrs.label.flatten {
                Ok(quote! {
                    ::fastmetrics::encoder::EncodeLabelSet::is_empty(&self.#ident)
                })
            } else {
                Ok(quote! {{
                    use ::fastmetrics::encoder::EncodeLabelValue;
                    EncodeLabelValue::skip_encoding(&self.#ident)
                }})
            }
        })
        .collect::<Result<Vec<_>>>()?;

    // Generate the `EncodeLabelSet` implementation
    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::encoder::EncodeLabelSet for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelSetEncoder) -> ::fastmetrics::error::Result<()> {
                use ::fastmetrics::encoder::EncodeLabel;

                #(#encode_stmts;)*

                ::core::result::Result::Ok(())
            }

            #[inline]
            fn is_empty(&self) -> bool {
                true #(&& #is_empty_exprs)*
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}
