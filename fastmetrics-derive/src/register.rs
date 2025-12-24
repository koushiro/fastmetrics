use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Error, Expr, ExprLit, Field, Fields, FieldsNamed, Lit, LitStr,
    Meta, MetaNameValue, Path, Result, Token, punctuated::Punctuated,
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
    let register_stmts = fields
        .into_iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().expect("fields must be named");
            let field_attrs = FieldAttributes::parse(field)?;

            // #[register(skip)] -> no encoding for this field
            if field_attrs.register.skip {
                return Ok(quote! { /* skip */ });
            }

            // #[register(flatten)] or #[register(subsystem)] -> encode nested or subsystem metrics
            // (both need to call register on the field)
            let is_flatten =
                field_attrs.register.flatten || field_attrs.register.subsystem.is_some();
            if is_flatten {
                return Ok(match &field_attrs.register.subsystem {
                    Some(subsystem_name) => {
                        let subsystem_expr = subsystem_name.to_token_stream();
                        quote! {
                            let subsystem = registry.subsystem(#subsystem_expr)?;
                            self.#field_ident.register(subsystem)?;
                        }
                    },
                    None => {
                        quote! {
                            self.#field_ident.register(registry)?;
                        }
                    },
                });
            }

            // #[register(rename = "...")] -> override metric name
            // Get the metric name from rename attribute or field ident
            let name = match &field_attrs.register.rename {
                Some(rename) => rename.to_token_stream(),
                None => {
                    let field_name = field_ident.to_string();
                    let name_lit_str = LitStr::new(&field_name, field_ident.span());
                    quote!(#name_lit_str)
                },
            };

            // Get help from help attribute or doc comments
            let help = match &field_attrs.register.help {
                Some(help) => help.to_token_stream(),
                None => {
                    let help_text = if field_attrs.docs.is_empty() {
                        String::new()
                    } else {
                        field_attrs.docs.join(" ")
                    };
                    let help_lit_str = LitStr::new(&help_text, proc_macro2::Span::call_site());
                    quote!(#help_lit_str)
                },
            };

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
                Some(UnitValue::StringValue(unit_str)) => {
                    let unit_expr = unit_str.to_token_stream();
                    quote! {
                        registry.register_with_unit(
                            #name,
                            #help,
                            ::fastmetrics::registry::Unit::Other((#unit_expr).into()),
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
            fn register(&self, registry: &mut ::fastmetrics::registry::Registry) -> ::core::result::Result<(), ::fastmetrics::error::Error> {
                #(#register_stmts)*

                ::core::result::Result::Ok(())
            }
        }
    };

    Ok(wrap_in_const(input, impl_block))
}

#[derive(Default)]
struct FieldAttributes {
    // #[register(...)]
    register: FieldRegisterAttribute,
    // multiple #[doc = "..."]
    docs: Vec<String>,
}

impl FieldAttributes {
    fn parse(field: &Field) -> Result<Self> {
        let mut field_attrs = FieldAttributes {
            register: FieldRegisterAttribute::default(),
            docs: extract_doc_comments(field),
        };

        // Phase 1: collect all #[register(...)] attributes (each #[register(...)] separately)
        let mut register_attrs: Vec<FieldRegisterAttribute> = Vec::new();
        for attr in &field.attrs {
            if attr.path().is_ident("register") {
                let register_attr = FieldRegisterAttribute::parse(attr)?;
                register_attrs.push(register_attr);
            }
        }

        // Phase 2: merge register attributes
        for attr in register_attrs {
            if attr.skip {
                if field_attrs.register.skip {
                    return Err(Error::new_spanned(field, "duplicated `skip` attribute"));
                }
                field_attrs.register.skip = true;
            }
            if attr.flatten {
                if field_attrs.register.flatten {
                    return Err(Error::new_spanned(field, "duplicated `flatten` attribute"));
                }
                field_attrs.register.flatten = true;
            }
            if let Some(subsystem) = attr.subsystem {
                if field_attrs.register.subsystem.is_some() {
                    return Err(Error::new_spanned(field, "duplicated `subsystem` attribute"));
                }
                field_attrs.register.subsystem = Some(subsystem);
            }
            if let Some(rename) = attr.rename {
                if field_attrs.register.rename.is_some() {
                    return Err(Error::new_spanned(field, "duplicated `rename` attribute"));
                }
                field_attrs.register.rename = Some(rename);
            }
            if let Some(help) = attr.help {
                if field_attrs.register.help.is_some() {
                    return Err(Error::new_spanned(field, "duplicated `help` attribute"));
                }
                field_attrs.register.help = Some(help);
            }
            if let Some(unit) = attr.unit {
                if field_attrs.register.unit.is_some() {
                    return Err(Error::new_spanned(field, "duplicated `unit` attribute"));
                }
                field_attrs.register.unit = Some(unit);
            }
        }

        // Phase 3: validate conflicts
        let register = &field_attrs.register;

        // Exclusive attributes: skip, flatten, subsystem must not coexist with each other
        let exclusive_count =
            (register.skip as u8) + (register.flatten as u8) + (register.subsystem.is_some() as u8);
        if exclusive_count > 1 {
            return Err(Error::new_spanned(
                field,
                "`skip`, `flatten`, `subsystem` attributes are mutually exclusive",
            ));
        }

        // If any exclusive attribute is present, it cannot coexist with non-exclusive attributes
        let has_non_exclusive =
            register.rename.is_some() || register.help.is_some() || register.unit.is_some();

        if register.skip && has_non_exclusive {
            return Err(Error::new_spanned(
                field,
                "`skip` attribute cannot coexist with other register attributes",
            ));
        }
        if register.flatten && has_non_exclusive {
            return Err(Error::new_spanned(
                field,
                "`flatten` attribute cannot coexist with other register attributes",
            ));
        }
        if register.subsystem.is_some() && has_non_exclusive {
            return Err(Error::new_spanned(
                field,
                "`subsystem` attribute cannot coexist with other register attributes",
            ));
        }

        Ok(field_attrs)
    }
}

/// Represents a parsed `#[register(...)]` attribute that controls how a field is registered with
/// the metrics registry.
/// This struct contains all possible configuration options that can be specified in the attribute.
#[derive(Default)]
struct FieldRegisterAttribute {
    // #[register(skip)]
    /// Whether to skip registering this field
    skip: bool,
    // #[register(flatten)]
    /// Whether to call the field's own `register` method instead of registering it directly.
    /// Used when a field contains nested metrics that should be registered individually.
    flatten: bool,
    // #[register(subsystem = "...")]
    /// Subsystem to register this nested struct into
    subsystem: Option<StringValue>,

    // #[register(rename = "...")]
    /// Custom name for the metric instead of field name
    rename: Option<StringValue>,
    // #[register(help = "...")]
    /// Custom help text that overrides doc comments
    help: Option<StringValue>,
    // #[register(unit(...)] or #[register(unit = "...")]
    /// Unit for the metric
    unit: Option<UnitValue>,
}

/// Represents a unit value which can be a path (e.g., Bytes) or a string value (e.g., "bytes")
enum UnitValue {
    /// Unit variant from the Unit enum (e.g., Bytes)
    Path(Path),
    /// Custom unit string literal or expression (e.g. "bytes" or UNIT_CONST)
    StringValue(StringValue),
}

impl FieldRegisterAttribute {
    /// Parse a single `#[register(...)]` attribute.
    fn parse(attr: &Attribute) -> Result<Self> {
        let mut register_attr = Self::default();

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in nested {
            match meta {
                // #[register(skip)]
                Meta::Path(path) if path.is_ident("skip") => {
                    if register_attr.skip {
                        return Err(Error::new_spanned(path, "duplicated `skip` attribute"));
                    }
                    if register_attr.flatten
                        || register_attr.subsystem.is_some()
                        || register_attr.rename.is_some()
                        || register_attr.help.is_some()
                        || register_attr.unit.is_some()
                    {
                        return Err(Error::new_spanned(
                            path,
                            "`skip` attribute cannot coexist with other register attributes",
                        ));
                    }
                    register_attr.skip = true;
                },

                // #[register(flatten)]
                Meta::Path(path) if path.is_ident("flatten") => {
                    if register_attr.flatten {
                        return Err(Error::new_spanned(path, "duplicated `flatten` attribute"));
                    }
                    if register_attr.skip
                        || register_attr.subsystem.is_some()
                        || register_attr.rename.is_some()
                        || register_attr.help.is_some()
                        || register_attr.unit.is_some()
                    {
                        return Err(Error::new_spanned(
                            path,
                            "`flatten` attribute cannot coexist with other register attributes",
                        ));
                    }
                    register_attr.flatten = true;
                },

                // #[register(subsystem = "...")]
                Meta::NameValue(nv) if nv.path.is_ident("subsystem") => {
                    if register_attr.subsystem.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `subsystem` attribute"));
                    }
                    if register_attr.skip
                        || register_attr.flatten
                        || register_attr.rename.is_some()
                        || register_attr.help.is_some()
                        || register_attr.unit.is_some()
                    {
                        return Err(Error::new_spanned(
                            nv,
                            "`subsystem` attribute cannot coexist with other register attributes",
                        ));
                    }
                    let subsystem = StringValue::from_expr(&nv.value)?;
                    register_attr.subsystem = Some(subsystem);
                },

                // #[register(rename = "...")]
                Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                    if register_attr.rename.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `rename` attribute"));
                    }
                    let rename = StringValue::from_expr(&nv.value)?;
                    register_attr.rename = Some(rename);
                },

                // #[register(help = "...")]
                Meta::NameValue(nv) if nv.path.is_ident("help") => {
                    if register_attr.help.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `help` attribute"));
                    }
                    let help = StringValue::from_expr(&nv.value)?;
                    register_attr.help = Some(help);
                },

                // #[register(unit = "...")
                Meta::NameValue(nv) if nv.path.is_ident("unit") => {
                    if register_attr.unit.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `unit` attribute"));
                    }
                    let value = StringValue::from_expr(&nv.value)?;
                    register_attr.unit = Some(UnitValue::StringValue(value));
                },
                // #[register(unit(...)]
                Meta::List(list) if list.path.is_ident("unit") => {
                    let path = list.parse_args::<Path>()?;
                    if register_attr.unit.is_some() {
                        return Err(Error::new_spanned(list, "duplicated `unit` attribute"));
                    }
                    register_attr.unit = Some(UnitValue::Path(path));
                },

                // unrecognized
                _ => {
                    return Err(Error::new_spanned(meta, "unrecognized register attribute"));
                },
            }
        }

        Ok(register_attr)
    }
}

/// Extract doc comments from field
fn extract_doc_comments(field: &Field) -> Vec<String> {
    let is_blank = |s: &str| -> bool { s.trim().is_empty() };

    // multiline comments (`/** ... */`) may have LFs (`\n`) in them,
    // we need to split, so we could handle the lines correctly
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
