use std::fmt;

/// Validation strategy for metric and label names at registry registration time.
///
/// - [`NameRule::Legacy`] enforces the legacy OpenMetrics ABNF name rules.
/// - [`NameRule::Utf8`] accepts UTF-8 names and rejects control/whitespace and
///   text-delimiter characters, while deferring escaping/scheme-specific
///   compatibility checks to exposition-time encoding.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum NameRule {
    /// Enforce legacy OpenMetrics name rules on registration.
    #[default]
    Legacy,
    /// Accept UTF-8 names on registration.
    Utf8,
}

/// Violations of the OpenMetrics ABNF for metric names.
#[derive(Clone, Debug)]
pub enum MetricNameViolation {
    /// The metric name is empty.
    Empty,
    /// The first character violates `metricname-initial-char`.
    InvalidFirstChar(char),
    /// Any subsequent character violates `metricname-char`.
    InvalidSubsequentChar(char),
}

impl fmt::Display for MetricNameViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("metric name must not be empty"),
            Self::InvalidFirstChar(ch) => {
                write!(
                    f,
                    "the first character '{ch}' is invalid for metric name; expected [A-Za-z_:]"
                )
            },
            Self::InvalidSubsequentChar(ch) => {
                write!(
                    f,
                    "the subsequent character '{ch}' is invalid for metric name; expected [A-Za-z0-9_:]"
                )
            },
        }
    }
}

/// Violations of the OpenMetrics ABNF for label names.
#[derive(Clone, Debug)]
pub enum LabelNameViolation {
    /// The label name is empty.
    Empty,
    /// The first character violates `label-name-initial-char`.
    InvalidFirstChar(char),
    /// Any subsequent character violates `label-name-char`.
    InvalidSubsequentChar(char),
}

impl fmt::Display for LabelNameViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("label name must not be empty"),
            Self::InvalidFirstChar(ch) => {
                write!(
                    f,
                    "the first character '{ch}' is invalid for label name; expected [A-Za-z_]"
                )
            },
            Self::InvalidSubsequentChar(ch) => {
                write!(
                    f,
                    "the subsequent character '{ch}' is invalid for label name; expected [A-Za-z0-9_]"
                )
            },
        }
    }
}

/// Violations of the OpenMetrics escaped-string rules for HELP text.
#[derive(Clone, Debug)]
pub enum HelpTextViolation {
    /// HELP text contains an unescaped line feed (LF) character.
    ContainsLineFeed,
    /// HELP text ends with a standalone backslash, which is not a valid escape sequence.
    DanglingEscape,
    /// HELP text contains a double quote that is not escaped as `\"`.
    UnescapedDoubleQuote,
}

impl fmt::Display for HelpTextViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContainsLineFeed => {
                f.write_str("help text must not contain line feed characters; escape them as \\n")
            },
            Self::DanglingEscape => f.write_str(
                "help text ends with a backslash that is not followed by another character",
            ),
            Self::UnescapedDoubleQuote => {
                f.write_str("double quotes inside help text must be escaped as \\\"")
            },
        }
    }
}

/// Violations of the OpenMetrics rules for unit strings.
#[derive(Clone, Debug)]
pub enum UnitViolation {
    /// Unit strings must not be empty.
    Empty,
    /// The first character violates `metricname-initial-char` for unit strings.
    InvalidFirstChar(char),
    /// Any subsequent character violates `metricname-char` for unit strings.
    InvalidSubsequentChar(char),
}

impl fmt::Display for UnitViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("unit strings must not be empty"),
            Self::InvalidFirstChar(ch) => {
                write!(
                    f,
                    "the first character '{ch}' is invalid for unit strings; expected [A-Za-z_:]"
                )
            },
            Self::InvalidSubsequentChar(ch) => {
                write!(
                    f,
                    "the subsequent character '{ch}' is invalid for unit strings; expected [A-Za-z0-9_:]"
                )
            },
        }
    }
}

fn validate_metric_name(name: &str, require_initial: bool) -> Result<(), MetricNameViolation> {
    if name.is_empty() {
        return Err(MetricNameViolation::Empty);
    }

    let mut chars = name.chars();
    let first = chars.next().expect("non-empty string has a first char");

    if require_initial {
        if !is_metricname_initial_char(first) {
            return Err(MetricNameViolation::InvalidFirstChar(first));
        }
    } else if !is_metricname_char(first) {
        return Err(MetricNameViolation::InvalidSubsequentChar(first));
    }

    for ch in chars {
        if !is_metricname_char(ch) {
            return Err(MetricNameViolation::InvalidSubsequentChar(ch));
        }
    }

    Ok(())
}

fn validate_utf8_name(name: &str) -> Result<(), MetricNameViolation> {
    if name.is_empty() {
        return Err(MetricNameViolation::Empty);
    }

    let mut chars = name.chars();
    let first = chars.next().expect("non-empty string has a first char");

    if first.is_whitespace() || first.is_control() || is_text_delimiter_char(first) {
        return Err(MetricNameViolation::InvalidFirstChar(first));
    }

    for ch in chars {
        if ch.is_whitespace() || ch.is_control() || is_text_delimiter_char(ch) {
            return Err(MetricNameViolation::InvalidSubsequentChar(ch));
        }
    }

    Ok(())
}

pub fn validate_metric_name_with_rule(
    name: &str,
    require_initial: bool,
    rule: NameRule,
) -> Result<(), MetricNameViolation> {
    match rule {
        NameRule::Legacy => validate_metric_name(name, require_initial),
        NameRule::Utf8 => validate_utf8_name(name),
    }
}

fn validate_label_name(name: &str) -> Result<(), LabelNameViolation> {
    if name.is_empty() {
        return Err(LabelNameViolation::Empty);
    }

    let mut chars = name.chars();
    let first = chars.next().expect("non-empty string has a first char");
    if !is_label_name_initial_char(first) {
        return Err(LabelNameViolation::InvalidFirstChar(first));
    }

    for ch in chars {
        if !is_label_name_char(ch) {
            return Err(LabelNameViolation::InvalidSubsequentChar(ch));
        }
    }

    Ok(())
}

fn validate_utf8_label_name(name: &str) -> Result<(), LabelNameViolation> {
    if name.is_empty() {
        return Err(LabelNameViolation::Empty);
    }

    let mut chars = name.chars();
    let first = chars.next().expect("non-empty string has a first char");

    if first.is_whitespace() || first.is_control() || is_text_delimiter_char(first) {
        return Err(LabelNameViolation::InvalidFirstChar(first));
    }

    for ch in chars {
        if ch.is_whitespace() || ch.is_control() || is_text_delimiter_char(ch) {
            return Err(LabelNameViolation::InvalidSubsequentChar(ch));
        }
    }

    Ok(())
}

#[inline]
fn is_text_delimiter_char(ch: char) -> bool {
    matches!(ch, '{' | '}' | ',' | '=' | '"' | '\\')
}

pub fn validate_label_name_with_rule(name: &str, rule: NameRule) -> Result<(), LabelNameViolation> {
    match rule {
        NameRule::Legacy => validate_label_name(name),
        NameRule::Utf8 => validate_utf8_label_name(name),
    }
}

pub(crate) fn is_legacy_metric_name(name: &str, require_initial: bool) -> bool {
    validate_metric_name(name, require_initial).is_ok()
}

pub(crate) fn is_legacy_label_name(name: &str) -> bool {
    validate_label_name(name).is_ok()
}

pub fn validate_help_text(help: &str) -> Result<(), HelpTextViolation> {
    let mut escape = false;
    for ch in help.chars() {
        if escape {
            escape = false;
            continue;
        }

        match ch {
            '\n' => return Err(HelpTextViolation::ContainsLineFeed),
            '"' => return Err(HelpTextViolation::UnescapedDoubleQuote),
            '\\' => escape = true,
            _ => {},
        }
    }

    if escape {
        return Err(HelpTextViolation::DanglingEscape);
    }

    Ok(())
}

pub fn validate_unit(unit: &str) -> Result<(), UnitViolation> {
    if unit.is_empty() {
        return Err(UnitViolation::Empty);
    }

    let mut chars = unit.chars();
    let first = chars.next().expect("non-empty string has a first char");
    if !is_metricname_initial_char(first) {
        return Err(UnitViolation::InvalidFirstChar(first));
    }

    for ch in chars {
        if !is_metricname_char(ch) {
            return Err(UnitViolation::InvalidSubsequentChar(ch));
        }
    }

    Ok(())
}

const fn is_metricname_initial_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == ':'
}

const fn is_metricname_char(ch: char) -> bool {
    is_metricname_initial_char(ch) || ch.is_ascii_digit()
}

const fn is_label_name_initial_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

const fn is_label_name_char(ch: char) -> bool {
    is_label_name_initial_char(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_metric_name() {
        assert!(validate_metric_name("valid_metric", true).is_ok());
        assert!(matches!(validate_metric_name("", true), Err(MetricNameViolation::Empty)));
        assert!(matches!(
            validate_metric_name("1bad", true),
            Err(MetricNameViolation::InvalidFirstChar('1'))
        ));
        assert!(matches!(
            validate_metric_name("bad-", true),
            Err(MetricNameViolation::InvalidSubsequentChar('-'))
        ));

        assert!(validate_metric_name("namespace", true).is_ok());
        assert!(matches!(
            validate_metric_name("1bad", true),
            Err(MetricNameViolation::InvalidFirstChar('1'))
        ));
        assert!(validate_metric_name("1subsystem", false).is_ok());
        assert!(validate_metric_name("subsystem2", false).is_ok());
        assert!(matches!(
            validate_metric_name("needs-hyphen", false),
            Err(MetricNameViolation::InvalidSubsequentChar('-'))
        ));
        assert!(matches!(validate_metric_name("", false), Err(MetricNameViolation::Empty)));
    }

    #[test]
    fn test_validate_metric_name_with_rule() {
        assert!(validate_metric_name_with_rule("valid_metric", true, NameRule::Legacy).is_ok());
        assert!(validate_metric_name_with_rule("utf8_指标", true, NameRule::Utf8).is_ok());
        assert!(matches!(
            validate_metric_name_with_rule("", true, NameRule::Utf8),
            Err(MetricNameViolation::Empty)
        ));
        assert!(matches!(
            validate_metric_name_with_rule("with space", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar(' '))
        ));
        assert!(matches!(
            validate_metric_name_with_rule("line\nbreak", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar('\n'))
        ));
        assert!(matches!(
            validate_metric_name_with_rule("metric{name", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar('{'))
        ));
        assert!(matches!(
            validate_metric_name_with_rule("metric=name", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar('='))
        ));
        assert!(matches!(
            validate_metric_name_with_rule("metric\"name", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar('"'))
        ));
        assert!(matches!(
            validate_metric_name_with_rule("metric,name", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar(','))
        ));
        assert!(matches!(
            validate_metric_name_with_rule("metric\\name", true, NameRule::Utf8),
            Err(MetricNameViolation::InvalidSubsequentChar('\\'))
        ));
    }

    #[test]
    fn test_validate_label_name() {
        assert!(validate_label_name("label_name").is_ok());
        assert!(matches!(validate_label_name(""), Err(LabelNameViolation::Empty)));
        assert!(matches!(
            validate_label_name("1bad"),
            Err(LabelNameViolation::InvalidFirstChar('1'))
        ));
        assert!(matches!(
            validate_label_name("bad-"),
            Err(LabelNameViolation::InvalidSubsequentChar('-'))
        ));
    }

    #[test]
    fn test_validate_label_name_with_rule() {
        assert!(validate_label_name_with_rule("label_name", NameRule::Legacy).is_ok());
        assert!(validate_label_name_with_rule("标签", NameRule::Utf8).is_ok());
        assert!(matches!(
            validate_label_name_with_rule("", NameRule::Utf8),
            Err(LabelNameViolation::Empty)
        ));
        assert!(matches!(
            validate_label_name_with_rule("with space", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar(' '))
        ));
        assert!(matches!(
            validate_label_name_with_rule("line\nbreak", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar('\n'))
        ));
        assert!(matches!(
            validate_label_name_with_rule("label{name", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar('{'))
        ));
        assert!(matches!(
            validate_label_name_with_rule("label=name", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar('='))
        ));
        assert!(matches!(
            validate_label_name_with_rule("label\"name", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar('"'))
        ));
        assert!(matches!(
            validate_label_name_with_rule("label,name", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar(','))
        ));
        assert!(matches!(
            validate_label_name_with_rule("label\\name", NameRule::Utf8),
            Err(LabelNameViolation::InvalidSubsequentChar('\\'))
        ));
    }

    #[test]
    fn test_validate_help_text() {
        assert!(validate_help_text("valid help text").is_ok());
        assert!(matches!(
            validate_help_text("has\nnewline"),
            Err(HelpTextViolation::ContainsLineFeed)
        ));
        assert!(matches!(
            validate_help_text("dangling \\"),
            Err(HelpTextViolation::DanglingEscape)
        ));
        assert!(matches!(
            validate_help_text("needs \" escape"),
            Err(HelpTextViolation::UnescapedDoubleQuote)
        ));
        assert!(validate_help_text("escaped \\\" quote and \\\\ slash").is_ok());
    }

    #[test]
    fn test_validate_unit() {
        assert!(validate_unit("seconds").is_ok());
        assert!(matches!(validate_unit(""), Err(UnitViolation::Empty)));
        assert!(matches!(validate_unit("-bad"), Err(UnitViolation::InvalidFirstChar('-'))));
        assert!(matches!(validate_unit("bad-"), Err(UnitViolation::InvalidSubsequentChar('-'))));
    }
}
