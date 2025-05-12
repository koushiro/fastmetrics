use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Attribute, Data, DeriveInput, Error, Expr, ExprLit, Field, Fields,
    FieldsNamed, Lit, LitStr, Meta, MetaNameValue, Path, Result, Token,
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
                let error = "#[derive(Register)] can only be used for structs with named fields.";
                return Err(Error::new_spanned(name, error));
            },
        },
        _ => {
            let error = "#[derive(Register)] can only be derived for structs";
            return Err(Error::new_spanned(name, error));
        },
    };

    // Generate register code for each field
    let register_calls = fields
        .into_iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().expect("fields must be named");
            let field_attrs = FieldAttributes::parse(field)?;

            // Skip field if marked with #[register(skip)]
            if field_attrs.register.skip {
                return Ok(quote! {});
            }

            // Flatten field if marked with #[register(flatten)]
            if field_attrs.register.flatten {
                return Ok(quote! {
                    self.#field_ident.register(registry)?;
                });
            }

            // Get the metric name from rename attribute or field ident
            let name = field_attrs.register.rename.unwrap_or(field_ident.to_string());

            // Get help from doc comments
            let docs = field_attrs.docs;
            let help = if docs.is_empty() { String::new() } else { docs.join(" ") };

            // Generate `register` code based on unit attribute
            let body = match &field_attrs.register.unit {
                Some(UnitValue::Path(unit_variant)) => {
                    quote! {
                        registry.register_with_unit(
                            #name,
                            #help,
                            ::fastmetrics::registry::Unit::#unit_variant,
                            self.#field_ident.clone(),
                        )?;
                    }
                },
                Some(UnitValue::LitStr(unit_str)) => {
                    quote! {
                        registry.register_with_unit(
                            #name,
                            #help,
                            ::fastmetrics::registry::Unit::Other(#unit_str.into()),
                            self.#field_ident.clone(),
                        )?;
                    }
                },
                None => {
                    quote! {
                        registry.register(
                            #name,
                            #help,
                            self.#field_ident.clone(),
                        )?;
                    }
                },
            };
            Ok(body)
        })
        .collect::<Result<Vec<_>>>()?;

    // Generate the `Register` trait implementation
    let impl_block = quote! {
        #[automatically_derived]
        impl #impl_generics ::fastmetrics::registry::Register for #name #ty_generics #where_clause {
            fn register(&self, registry: &mut ::fastmetrics::registry::Registry) -> ::core::result::Result<(), ::fastmetrics::registry::RegistryError> {
                #(#register_calls)*

                ::core::result::Result::Ok(())
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}

#[derive(Default)]
struct FieldAttributes {
    // #[register(...)]
    register: RegisterAttribute,
    // multiple #[doc = "..."]
    docs: Vec<String>,
}

impl FieldAttributes {
    fn parse(field: &Field) -> Result<Self> {
        let mut field_attrs = FieldAttributes::default();

        let docs = extract_doc_comments(field);
        field_attrs.docs = docs;

        for attr in &field.attrs {
            if attr.path().is_ident("register") {
                let register_attr = parse_register_attr(attr)?;

                if let Some(rename) = register_attr.rename {
                    if field_attrs.register.rename.is_some() {
                        return Err(Error::new_spanned(attr, "duplicated `rename` attribute"));
                    }
                    field_attrs.register.rename = Some(rename);
                }

                if let Some(unit) = register_attr.unit {
                    if field_attrs.register.unit.is_some() {
                        return Err(Error::new_spanned(attr, "duplicated `unit` attribute"));
                    }
                    field_attrs.register.unit = Some(unit);
                }

                if register_attr.flatten {
                    if field_attrs.register.flatten {
                        return Err(Error::new_spanned(attr, "duplicated `flatten` attribute"));
                    }
                    if field_attrs.register.rename.is_some()
                        || field_attrs.register.unit.is_some()
                        || field_attrs.register.skip
                    {
                        return Err(Error::new_spanned(
                            attr,
                            "`flatten` attribute cannot coexist with other attributes",
                        ));
                    }
                    field_attrs.register.flatten = true;
                }

                if register_attr.skip {
                    if field_attrs.register.skip {
                        return Err(Error::new_spanned(attr, "duplicated `skip` attribute"));
                    }
                    if field_attrs.register.rename.is_some()
                        || field_attrs.register.unit.is_some()
                        || field_attrs.register.flatten
                    {
                        return Err(Error::new_spanned(
                            attr,
                            "`skip` attribute cannot coexist with other attributes",
                        ));
                    }
                    field_attrs.register.skip = true;
                }
            }
        }

        Ok(field_attrs)
    }
}

/// Represents a parsed `#[register(...)]` attribute that controls how a field is registered with
/// the metrics registry.
/// This struct contains all possible configuration options that can be specified in the attribute.
#[derive(Default)]
struct RegisterAttribute {
    // #[register(flatten)]
    /// Whether to call the field's own `register` method instead of registering it directly.
    /// Used when a field contains nested metrics that should be registered individually.
    flatten: bool,
    // #[register(skip)]
    /// Whether to skip registering this field
    skip: bool,
    // #[register(rename = "...")]
    /// Custom name for the metric instead of field name
    rename: Option<String>,
    // #[register(unit(...)] or #[register(unit = "...")]
    /// Unit for the metric
    unit: Option<UnitValue>,
}

/// Represents a unit value which can be a path (e.g. Bytes) or a string literal (e.g. "bytes")
enum UnitValue {
    /// Unit variant from the Unit enum (e.g. Bytes)
    Path(Path),
    /// Custom unit string (e.g. "bytes")
    LitStr(LitStr),
}

/// Parse a register attribute.
fn parse_register_attr(attr: &Attribute) -> Result<RegisterAttribute> {
    let mut register_attr = RegisterAttribute::default();

    let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in nested {
        match meta {
            // #[register(flatten)]
            Meta::Path(path) if path.is_ident("flatten") => {
                if register_attr.flatten {
                    return Err(Error::new_spanned(path, "duplicated `flatten` attribute"));
                }
                register_attr.flatten = true;
            },

            // #[register(skip)]
            Meta::Path(path) if path.is_ident("skip") => {
                if register_attr.skip {
                    return Err(Error::new_spanned(path, "duplicated `skip` attribute"));
                }
                register_attr.skip = true;
            },

            // #[register(rename = "...")]
            Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                if let Expr::Lit(ExprLit { lit: Lit::Str(ref s), .. }) = nv.value {
                    if register_attr.rename.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `rename` attribute"));
                    }
                    register_attr.rename = Some(s.value())
                } else {
                    return Err(Error::new_spanned(nv.value, "expected a literal string"));
                }
            },

            // #[register(unit = "...")
            Meta::NameValue(nv) if nv.path.is_ident("unit") => {
                if let Expr::Lit(ExprLit { lit: Lit::Str(ref s), .. }) = nv.value {
                    if register_attr.unit.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `unit` attribute"));
                    }
                    register_attr.unit = Some(UnitValue::LitStr(s.clone()))
                } else {
                    return Err(Error::new_spanned(nv.value, "expected a literal string"));
                }
            },
            // #[register(unit(...)]
            Meta::List(list) if list.path.is_ident("unit") => {
                let path = list.parse_args::<Path>()?;
                if register_attr.unit.is_some() {
                    return Err(Error::new_spanned(list, "duplicated `unit` attribute"));
                }
                register_attr.unit = Some(UnitValue::Path(path));
            },

            _ => {
                return Err(Error::new_spanned(meta, "unrecognized register attribute"));
            },
        }
    }

    Ok(register_attr)
}

/// Extract doc comments from field
fn extract_doc_comments(field: &Field) -> Vec<String> {
    let is_blank = |s: &str| -> bool { s.trim().is_empty() };

    // multiline comments (`/** ... */`) may have LFs (`\n`) in them,
    // we need to split so we could handle the lines correctly
    //
    // we also need to remove leading and trailing blank lines
    let mut lines = field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            // non #[doc = "..."] attributes are not our concern
            // we leave them for rustc to handle
            match &attr.meta {
                Meta::NameValue(MetaNameValue {
                    value: Expr::Lit(ExprLit { lit: Lit::Str(s), .. }),
                    ..
                }) => Some(s.value()),
                _ => None,
            }
        })
        .skip_while(|s| is_blank(s))
        .flat_map(|s| {
            s.split('\n')
                .map(|s| s.trim_start().to_owned())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    while let Some(true) = lines.last().map(|s| is_blank(s)) {
        lines.pop();
    }

    lines
}
