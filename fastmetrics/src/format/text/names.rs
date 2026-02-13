use std::{borrow::Cow, fmt::Write as _};

use super::{EscapingScheme, config::NamePolicy};
use crate::{
    error::{Error, Result},
    registry::{is_legacy_label_name, is_legacy_metric_name},
};

#[derive(Clone, Copy)]
enum NameKind {
    Metric,
    Label,
}

impl NameKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Metric => "metric",
            Self::Label => "label",
        }
    }

    fn is_legacy_name(self, name: &str) -> bool {
        match self {
            Self::Metric => is_legacy_metric_name(name, true),
            Self::Label => is_legacy_label_name(name),
        }
    }

    fn is_legacy_char(self, ch: char, is_first: bool) -> bool {
        match self {
            Self::Metric => {
                if is_first {
                    is_legacy_metric_initial_char(ch)
                } else {
                    is_legacy_metric_char(ch)
                }
            },
            Self::Label => {
                if is_first {
                    is_legacy_label_initial_char(ch)
                } else {
                    is_legacy_label_char(ch)
                }
            },
        }
    }
}

pub(super) fn escape_metric_name<'a>(
    name: Cow<'a, str>,
    policy: NamePolicy,
) -> Result<Cow<'a, str>> {
    escape_name(name, policy, NameKind::Metric)
}

pub(super) fn escape_label_name<'a>(name: &'a str, policy: NamePolicy) -> Result<Cow<'a, str>> {
    escape_name(Cow::Borrowed(name), policy, NameKind::Label)
}

fn escape_name<'a>(name: Cow<'a, str>, policy: NamePolicy, kind: NameKind) -> Result<Cow<'a, str>> {
    match policy {
        NamePolicy::Legacy => {
            if kind.is_legacy_name(name.as_ref()) {
                Ok(name)
            } else {
                Err(Error::invalid(format!(
                    "{kind_name} name `{name}` is not valid for legacy text profiles",
                    kind_name = kind.as_str()
                )))
            }
        },
        NamePolicy::V1Escaping(scheme) => match scheme {
            EscapingScheme::AllowUtf8 => Ok(name),
            EscapingScheme::Underscores => Ok(escape_underscores(name, kind)),
            EscapingScheme::Dots => Ok(escape_dots(name, kind)),
            EscapingScheme::Values => Ok(escape_values(name, kind)),
        },
    }
}

fn escape_underscores<'a>(name: Cow<'a, str>, kind: NameKind) -> Cow<'a, str> {
    if kind.is_legacy_name(name.as_ref()) {
        return name;
    }

    let escaped = name
        .chars()
        .enumerate()
        .map(|(idx, ch)| if kind.is_legacy_char(ch, idx == 0) { ch } else { '_' })
        .collect::<String>();

    Cow::Owned(escaped)
}

fn escape_dots<'a>(name: Cow<'a, str>, kind: NameKind) -> Cow<'a, str> {
    let needs_rewrite = name
        .chars()
        .enumerate()
        .any(|(idx, ch)| ch == '.' || ch == '_' || !kind.is_legacy_char(ch, idx == 0));

    if !needs_rewrite {
        return name;
    }

    let mut escaped = String::with_capacity(name.len() + 8);
    for (idx, ch) in name.chars().enumerate() {
        match ch {
            '.' => escaped.push_str("_dot_"),
            '_' => escaped.push_str("__"),
            _ if kind.is_legacy_char(ch, idx == 0) => escaped.push(ch),
            _ => escaped.push('_'),
        }
    }

    Cow::Owned(escaped)
}

fn escape_values<'a>(name: Cow<'a, str>, kind: NameKind) -> Cow<'a, str> {
    let mut escaped = String::with_capacity(name.len() + 8);
    escaped.push_str("U__");

    for (idx, ch) in name.chars().enumerate() {
        match ch {
            '_' => escaped.push_str("__"),
            _ if kind.is_legacy_char(ch, idx == 0) => escaped.push(ch),
            _ => {
                escaped.push('_');
                write!(&mut escaped, "{:X}", ch as u32)
                    .expect("writing UTF-8 codepoint into String should not fail");
                escaped.push('_');
            },
        }
    }

    Cow::Owned(escaped)
}

const fn is_legacy_metric_initial_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || matches!(ch, '_' | ':')
}

const fn is_legacy_metric_char(ch: char) -> bool {
    is_legacy_metric_initial_char(ch) || ch.is_ascii_digit()
}

const fn is_legacy_label_initial_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

const fn is_legacy_label_char(ch: char) -> bool {
    is_legacy_label_initial_char(ch) || ch.is_ascii_digit()
}
