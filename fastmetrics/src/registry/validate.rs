use std::fmt;

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
            Self::Empty => f.write_str("metric names must not be empty"),
            Self::InvalidFirstChar(ch) => {
                write!(f, "the first character '{ch}' is invalid; expected [A-Za-z_:]")
            },
            Self::InvalidSubsequentChar(ch) => {
                write!(f, "the subsequent character '{ch}' is invalid; expected [A-Za-z0-9_:]")
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
            Self::Empty => f.write_str("label names must not be empty"),
            Self::InvalidFirstChar(ch) => {
                write!(f, "the first character '{ch}' is invalid; expected [A-Za-z_]")
            },
            Self::InvalidSubsequentChar(ch) => {
                write!(f, "the subsequent character '{ch}' is invalid; expected [A-Za-z0-9_]")
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
    /// Unit strings may only contain characters allowed by `metricname-char`.
    InvalidChar(char),
}

impl fmt::Display for UnitViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("unit strings must not be empty"),
            Self::InvalidChar(ch) => {
                write!(
                    f,
                    "the character '{ch}' is invalid for unit strings; expected [A-Za-z0-9_:]"
                )
            },
        }
    }
}

pub fn validate_metric_name(name: &str) -> Result<(), MetricNameViolation> {
    validate_metric_name_prefix(name)
}

pub fn validate_metric_name_prefix(prefix: &str) -> Result<(), MetricNameViolation> {
    validate_metric_name_chars(prefix, true)
}

pub fn validate_metric_name_component(component: &str) -> Result<(), MetricNameViolation> {
    validate_metric_name_chars(component, false)
}

fn validate_metric_name_chars(
    name: &str,
    require_initial: bool,
) -> Result<(), MetricNameViolation> {
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

pub fn validate_label_name(name: &str) -> Result<(), LabelNameViolation> {
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
        return Err(UnitViolation::InvalidChar(first));
    }

    for ch in chars {
        if !is_metricname_char(ch) {
            return Err(UnitViolation::InvalidChar(ch));
        }
    }

    Ok(())
}

fn is_metricname_initial_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || ch == ':'
}

fn is_metricname_char(ch: char) -> bool {
    is_metricname_initial_char(ch) || ch.is_ascii_digit()
}

fn is_label_name_initial_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_label_name_char(ch: char) -> bool {
    is_label_name_initial_char(ch) || ch.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_metric_name() {
        assert!(validate_metric_name("valid_metric").is_ok());
        assert!(matches!(validate_metric_name(""), Err(MetricNameViolation::Empty)));
        assert!(matches!(
            validate_metric_name("1bad"),
            Err(MetricNameViolation::InvalidFirstChar('1'))
        ));
        assert!(matches!(
            validate_metric_name("bad-"),
            Err(MetricNameViolation::InvalidSubsequentChar('-'))
        ));
    }

    #[test]
    fn test_validate_metric_name_components() {
        assert!(validate_metric_name_prefix("namespace").is_ok());
        assert!(matches!(
            validate_metric_name_prefix("1bad"),
            Err(MetricNameViolation::InvalidFirstChar('1'))
        ));
        assert!(validate_metric_name_component("1subsystem").is_ok());
        assert!(validate_metric_name_component("subsystem2").is_ok());
        assert!(matches!(
            validate_metric_name_component("needs-hyphen"),
            Err(MetricNameViolation::InvalidSubsequentChar('-'))
        ));
        assert!(matches!(validate_metric_name_component(""), Err(MetricNameViolation::Empty)));
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
        assert!(matches!(validate_unit("-bad"), Err(UnitViolation::InvalidChar('-'))));
    }
}
