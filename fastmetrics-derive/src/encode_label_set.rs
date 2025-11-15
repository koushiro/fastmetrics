use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Error, Field, Fields, FieldsNamed, Meta, Result, Token,
    punctuated::Punctuated,
};

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

    // Process all fields with #[label(...)] attributes
    let parsed_fields = fields
        .iter()
        .map(|field| Ok((field, FieldAttributes::parse(field)?)))
        .collect::<Result<Vec<_>>>()?;

    let encode_stmts = parsed_fields
        .iter()
        .map(|(field, attrs)| {
            let ident = field.ident.as_ref().expect("fields must be named");
            let ident_str = ident.to_string();

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

            // default: encode as ("field_name", &self.field)
            Ok(quote! {
                encoder.encode(&(#ident_str, &self.#ident))?
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

    // Generate the `EncodeLabelSet` trait implementation
    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::encoder::EncodeLabelSet for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut dyn ::fastmetrics::encoder::LabelSetEncoder) -> ::core::fmt::Result {
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

#[derive(Default)]
struct FieldAttributes {
    // #[label(...)]
    label: FieldLabelAttribute,
}

impl FieldAttributes {
    fn parse(field: &Field) -> Result<Self> {
        let mut field_attrs = FieldAttributes::default();

        for attr in &field.attrs {
            if attr.path().is_ident("label") {
                let label_attr = FieldLabelAttribute::parse(attr)?;

                if label_attr.skip {
                    if field_attrs.label.skip {
                        return Err(Error::new_spanned(attr, "duplicated `skip` attribute"));
                    }
                    if field_attrs.label.flatten {
                        return Err(Error::new_spanned(
                            attr,
                            "`skip` attribute cannot coexist with `flatten` attribute",
                        ));
                    }
                    field_attrs.label.skip = true;
                }

                if label_attr.flatten {
                    if field_attrs.label.flatten {
                        return Err(Error::new_spanned(attr, "duplicated `flatten` attribute"));
                    }
                    if field_attrs.label.skip {
                        return Err(Error::new_spanned(
                            attr,
                            "`flatten` attribute cannot coexist with `skip` attribute",
                        ));
                    }
                    field_attrs.label.flatten = true;
                }
            }
        }

        Ok(field_attrs)
    }
}

#[derive(Default)]
struct FieldLabelAttribute {
    // #[label(skip)]
    skip: bool,
    // #[label(flatten)]
    flatten: bool,
}

impl FieldLabelAttribute {
    /// Parse a `#[label(...)]` attribute.
    fn parse(attr: &Attribute) -> Result<Self> {
        let mut label_attr = Self::default();

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in nested {
            match meta {
                // #[label(skip)]
                Meta::Path(path) if path.is_ident("skip") => {
                    if label_attr.skip {
                        return Err(Error::new_spanned(path, "duplicated `skip` attribute"));
                    }
                    label_attr.skip = true;
                },

                // #[label(flatten)]
                Meta::Path(path) if path.is_ident("flatten") => {
                    if label_attr.flatten {
                        return Err(Error::new_spanned(path, "duplicated `flatten` attribute"));
                    }
                    label_attr.flatten = true;
                },

                _ => {
                    return Err(Error::new_spanned(meta, "unrecognized label attribute"));
                },
            }
        }

        Ok(label_attr)
    }
}
