use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result};

use crate::{encode_label_set, label_set_schema};

pub fn expand_derive(input: &DeriveInput) -> Result<TokenStream> {
    ensure_named_struct(input)?;

    let encode_impl = encode_label_set::expand_derive(input)?;
    let schema_impl = label_set_schema::expand_derive(input)?;

    Ok(quote! {
        #encode_impl
        #schema_impl
    })
}

fn ensure_named_struct(input: &DeriveInput) -> Result<()> {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(_) => Ok(()),
            _ => {
                let error =
                    "#[derive(LabelSet)] can only be derived for structs with named fields.";
                Err(Error::new_spanned(&input.ident, error))
            },
        },
        _ => {
            let error = "#[derive(LabelSet)] can only be derived for structs with named fields.";
            Err(Error::new_spanned(&input.ident, error))
        },
    }
}
