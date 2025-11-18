use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Error, Field, Fields, FieldsNamed, Meta, Result, Token,
    punctuated::Punctuated,
};

use crate::utils::{StringValue, wrap_in_const};

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

        // Phase 1: collect all #[label(...)] attributes
        let mut label_attrs: Vec<FieldLabelAttribute> = Vec::new();
        for attr in &field.attrs {
            if attr.path().is_ident("label") {
                let label_attr = FieldLabelAttribute::parse(attr)?;
                label_attrs.push(label_attr);
            }
        }

        // Phase 2: merge attributes
        for attr in label_attrs {
            if attr.skip {
                if field_attrs.label.skip {
                    return Err(Error::new_spanned(field, "duplicated `skip` attribute"));
                }
                field_attrs.label.skip = true;
            }
            if attr.flatten {
                if field_attrs.label.flatten {
                    return Err(Error::new_spanned(field, "duplicated `flatten` attribute"));
                }
                field_attrs.label.flatten = true;
            }
            if let Some(rename) = attr.rename {
                if field_attrs.label.rename.is_some() {
                    return Err(Error::new_spanned(field, "duplicated `rename` attribute"));
                }
                field_attrs.label.rename = Some(rename);
            }
        }

        // Phase 3: validate conflicts
        let label = &field_attrs.label;

        // Exclusive attributes: skip, flatten are mutually exclusive
        let exclusive_count = (label.skip as u8) + (label.flatten as u8);
        if exclusive_count > 1 {
            return Err(Error::new_spanned(
                field,
                "`skip`, `flatten` attributes are mutually exclusive",
            ));
        }

        // Non-exclusive: rename
        let has_non_exclusive = label.rename.is_some();

        if label.skip && has_non_exclusive {
            return Err(Error::new_spanned(
                field,
                "`skip` attribute cannot coexist with other label attributes",
            ));
        }
        if label.flatten && has_non_exclusive {
            return Err(Error::new_spanned(
                field,
                "`flatten` attribute cannot coexist with other label attributes",
            ));
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
    // #[label(rename = "...")]
    rename: Option<StringValue>,
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
                    if label_attr.flatten || label_attr.rename.is_some() {
                        return Err(Error::new_spanned(
                            path,
                            "`skip` attribute cannot coexist with other label attributes",
                        ));
                    }
                    label_attr.skip = true;
                },

                // #[label(flatten)]
                Meta::Path(path) if path.is_ident("flatten") => {
                    if label_attr.flatten {
                        return Err(Error::new_spanned(path, "duplicated `flatten` attribute"));
                    }
                    if label_attr.skip || label_attr.rename.is_some() {
                        return Err(Error::new_spanned(
                            path,
                            "`flatten` attribute cannot coexist with other label attributes",
                        ));
                    }
                    label_attr.flatten = true;
                },

                // #[label(rename = "...")]
                Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                    if label_attr.rename.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `rename` attribute"));
                    }
                    let rename = StringValue::from_expr(&nv.value)?;
                    label_attr.rename = Some(rename);
                },

                _ => {
                    return Err(Error::new_spanned(meta, "unrecognized label attribute"));
                },
            }
        }

        Ok(label_attr)
    }
}
