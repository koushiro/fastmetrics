use syn::{Attribute, Error, Field, Meta, Result, Token, punctuated::Punctuated};

use crate::utils::StringValue;

/// Aggregates all supported `#[label(...)]` attributes found on a field.
#[derive(Default, Clone)]
pub struct LabelAttributes {
    /// Metadata parsed from the `#[label(...)]` attribute group.
    pub label: LabelAttribute,
}

impl LabelAttributes {
    /// Parses every `#[label(...)]` attribute that appears on the provided `field`.
    pub fn parse(field: &Field) -> Result<Self> {
        let mut attrs = LabelAttributes::default();

        // Phase 1: collect all #[label(...)] attributes
        let label_attrs = field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("label"))
            .map(LabelAttribute::parse)
            .collect::<Result<Vec<_>>>()?;

        // Phase 2: merge attributes
        for attr in label_attrs {
            if attr.skip {
                if attrs.label.skip {
                    return Err(Error::new_spanned(field, "duplicated `skip` attribute"));
                }
                attrs.label.skip = true;
            }
            if attr.flatten {
                if attrs.label.flatten {
                    return Err(Error::new_spanned(field, "duplicated `flatten` attribute"));
                }
                attrs.label.flatten = true;
            }
            if let Some(rename) = attr.rename {
                if attrs.label.rename.is_some() {
                    return Err(Error::new_spanned(field, "duplicated `rename` attribute"));
                }
                attrs.label.rename = Some(rename);
            }
        }

        // Phase 3: validate conflicts
        let label = &attrs.label;

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

        Ok(attrs)
    }
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

impl LabelAttribute {
    /// Parse a `#[label(...)]` attribute.
    fn parse(attr: &Attribute) -> Result<Self> {
        let mut parsed = Self::default();

        let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
        for meta in nested {
            match meta {
                // #[label(skip)]
                Meta::Path(path) if path.is_ident("skip") => {
                    if parsed.skip {
                        return Err(Error::new_spanned(path, "duplicated `skip` attribute"));
                    }
                    if parsed.flatten || parsed.rename.is_some() {
                        return Err(Error::new_spanned(
                            path,
                            "`skip` attribute cannot coexist with other label attributes",
                        ));
                    }
                    parsed.skip = true;
                },

                // #[label(flatten)]
                Meta::Path(path) if path.is_ident("flatten") => {
                    if parsed.flatten {
                        return Err(Error::new_spanned(path, "duplicated `flatten` attribute"));
                    }
                    if parsed.skip || parsed.rename.is_some() {
                        return Err(Error::new_spanned(
                            path,
                            "`flatten` attribute cannot coexist with other label attributes",
                        ));
                    }
                    parsed.flatten = true;
                },

                // #[label(rename = "...")]
                Meta::NameValue(nv) if nv.path.is_ident("rename") => {
                    if parsed.rename.is_some() {
                        return Err(Error::new_spanned(nv, "duplicated `rename` attribute"));
                    }
                    let rename = StringValue::from_expr(&nv.value)?;
                    parsed.rename = Some(rename);
                },

                // unrecognized label attribute
                _ => {
                    return Err(Error::new_spanned(meta, "unrecognized label attribute"));
                },
            }
        }

        Ok(parsed)
    }
}
