use crate::utils::StringValue;
use syn::{Attribute, Field, Meta, Result, Token, punctuated::Punctuated};

/// Aggregates all supported `#[label(...)]` attributes found on a field.
#[derive(Default, Clone)]
pub struct LabelAttributes {
    /// Metadata parsed from the `#[label(...)]` attribute group.
    pub label: LabelAttribute,
}

/// Individual settings supported within a `#[label(...)]` attribute.
#[derive(Default, Clone)]
pub struct LabelAttribute {
    /// Skips the field entirely when encoding labels.
    pub skip: bool,
    /// Flattens the field, delegating to the nested label set implementation.
    pub flatten: bool,
    /// Overrides the generated label name.
    pub rename: Option<StringValue>,
}

impl LabelAttributes {
    /// Parses every `#[label(...)]` attribute that appears on the provided `field`.
    pub fn parse(field: &Field) -> Result<Self> {
        let mut attrs = LabelAttributes::default();

        let parsed_groups = field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("label"))
            .map(parse_label_meta)
            .collect::<Result<Vec<_>>>()?;

        for group in parsed_groups {
            attrs.merge(field, group)?;
        }

        Ok(attrs)
    }

    fn merge(&mut self, field: &Field, other: LabelAttribute) -> Result<()> {
        if other.skip {
            if self.label.skip {
                return Err(syn::Error::new_spanned(field, "duplicated `skip` attribute"));
            }
            self.label.skip = true;
        }

        if other.flatten {
            if self.label.flatten {
                return Err(syn::Error::new_spanned(field, "duplicated `flatten` attribute"));
            }
            self.label.flatten = true;
        }

        if let Some(rename) = other.rename {
            if self.label.rename.is_some() {
                return Err(syn::Error::new_spanned(field, "duplicated `rename` attribute"));
            }
            self.label.rename = Some(rename);
        }

        // Validate exclusivity constraints.
        let exclusive_count = self.label.skip as u8 + self.label.flatten as u8;
        if exclusive_count > 1 {
            return Err(syn::Error::new_spanned(
                field,
                "`skip`, `flatten` attributes are mutually exclusive",
            ));
        }

        if self.label.skip && self.label.rename.is_some() {
            return Err(syn::Error::new_spanned(
                field,
                "`skip` attribute cannot be combined with `rename`",
            ));
        }

        if self.label.flatten && self.label.rename.is_some() {
            return Err(syn::Error::new_spanned(
                field,
                "`flatten` attribute cannot be combined with `rename`",
            ));
        }

        Ok(())
    }
}

fn parse_label_meta(attr: &Attribute) -> Result<LabelAttribute> {
    let mut parsed = LabelAttribute::default();

    let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
    for meta in nested {
        match meta {
            Meta::Path(path) if path.is_ident("skip") => {
                if parsed.skip {
                    return Err(syn::Error::new_spanned(path, "duplicated `skip` attribute"));
                }
                if parsed.flatten || parsed.rename.is_some() {
                    return Err(syn::Error::new_spanned(
                        path,
                        "`skip` attribute cannot coexist with other label attributes",
                    ));
                }
                parsed.skip = true;
            },
            Meta::Path(path) if path.is_ident("flatten") => {
                if parsed.flatten {
                    return Err(syn::Error::new_spanned(path, "duplicated `flatten` attribute"));
                }
                if parsed.skip || parsed.rename.is_some() {
                    return Err(syn::Error::new_spanned(
                        path,
                        "`flatten` attribute cannot coexist with other label attributes",
                    ));
                }
                parsed.flatten = true;
            },
            Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                if parsed.rename.is_some() {
                    return Err(syn::Error::new_spanned(nv, "duplicated `rename` attribute"));
                }
                parsed.rename = Some(StringValue::from_expr(&nv.value)?);
            },
            _ => {
                return Err(syn::Error::new_spanned(meta, "unrecognized label attribute"));
            },
        }
    }

    Ok(parsed)
}
