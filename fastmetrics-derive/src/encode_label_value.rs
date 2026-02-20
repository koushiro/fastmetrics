use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result, parse_quote};

use crate::utils::wrap_in_const;

pub fn expand_derive(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let impl_block = match &input.data {
        Data::Enum(data_enum) => {
            if data_enum.variants.is_empty() {
                let error = "#[derive(EncodeLabelValue)] requires at least one variant in the enum";
                return Err(Error::new_spanned(name, error));
            }

            let variant_arms = data_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_name = &variant.ident;

                    match &variant.fields {
                        Fields::Unit => {},
                        _ => {
                            let error = "#[derive(EncodeLabelValue)] can only be used for enums with unit variants";
                            return Err(Error::new_spanned(variant, error));
                        },
                    }

                    let variant_str = variant_name.to_string();

                    Ok(quote! {
                        #name::#variant_name => encoder.encode_str_value(&#variant_str)?
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            quote! {
                #[automatically_derived]
                impl #impl_generics ::fastmetrics::encoder::EncodeLabelValue for #name #ty_generics #where_clause {
                    fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelEncoder) -> ::fastmetrics::error::Result<()> {
                        match self {
                            #(#variant_arms,)*
                        }

                        ::core::result::Result::Ok(())
                    }
                }
            }
        },
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field_ty = &fields.unnamed[0].ty;
                let mut generics_with_bound = input.generics.clone();
                generics_with_bound
                    .make_where_clause()
                    .predicates
                    .push(parse_quote!(#field_ty: ::fastmetrics::encoder::EncodeLabelValue));
                let (impl_generics, ty_generics, where_clause) =
                    generics_with_bound.split_for_impl();

                quote! {
                    #[automatically_derived]
                    impl #impl_generics ::fastmetrics::encoder::EncodeLabelValue for #name #ty_generics #where_clause {
                        fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelEncoder) -> ::fastmetrics::error::Result<()> {
                            ::fastmetrics::encoder::EncodeLabelValue::encode(&self.0, encoder)
                        }

                        fn skip_encoding(&self) -> bool {
                            ::fastmetrics::encoder::EncodeLabelValue::skip_encoding(&self.0)
                        }
                    }
                }
            },
            Fields::Unnamed(_) => {
                let error = "#[derive(EncodeLabelValue)] on structs requires a single-field tuple struct (newtype)";
                return Err(Error::new_spanned(name, error));
            },
            _ => {
                let error = "#[derive(EncodeLabelValue)] can only be used on enums or single-field tuple structs";
                return Err(Error::new_spanned(name, error));
            },
        },
        _ => {
            let error = "#[derive(EncodeLabelValue)] can only be used on enums or single-field tuple structs";
            return Err(Error::new_spanned(name, error));
        },
    };

    Ok(wrap_in_const(input, impl_block))
}
