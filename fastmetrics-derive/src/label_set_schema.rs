use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, FieldsNamed, Result};

use crate::{label_attributes::LabelAttributes, utils::wrap_in_const};

/// Expands `#[derive(LabelSetSchema)]` for structs with named fields.
pub fn expand_derive(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Only works for structs with named fields.
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => named,
            _ => {
                let error =
                    "#[derive(LabelSetSchema)] can only be derived for structs with named fields.";
                return Err(Error::new_spanned(name, error));
            },
        },
        _ => {
            let error =
                "#[derive(LabelSetSchema)] can only be derived for structs with named fields.";
            return Err(Error::new_spanned(name, error));
        },
    };

    let parsed_fields = fields
        .iter()
        .map(|field| Ok((field, LabelAttributes::parse(field)?)))
        .collect::<Result<Vec<_>>>()?;

    let push_stmts = parsed_fields.iter().map(|(field, attrs)| {
        let ident = field.ident.as_ref().expect("fields must be named");

        if attrs.label.skip {
            quote! { /*  skip */ }
        } else if attrs.label.flatten {
            let ty = &field.ty;
            quote! {
                if let Some(schema) = <#ty as ::fastmetrics::raw::LabelSetSchema>::names() {
                    names.extend(schema.iter().copied());
                }
            }
        } else {
            let field_name_tokens = if let Some(rename) = &attrs.label.rename {
                rename.to_token_stream()
            } else {
                let ident_str = ident.to_string();
                quote!(#ident_str)
            };

            quote! {
                names.push(#field_name_tokens);
            }
        }
    });

    let names_body = quote! {{
        let mut names: ::std::vec::Vec<&'static str> = ::std::vec::Vec::new();
        #(#push_stmts)*
        names
    }};

    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::raw::LabelSetSchema for #name #ty_generics #where_clause {
            fn names() -> Option<&'static [&'static str]> {
                static NAMES: ::std::sync::OnceLock<Option<&'static [&'static str]>> =
                    ::std::sync::OnceLock::new();

                *NAMES.get_or_init(|| {
                    let names = #names_body;
                    if names.is_empty() {
                        None
                    } else {
                        Some(::std::boxed::Box::leak(names.into_boxed_slice()))
                    }
                })
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}
