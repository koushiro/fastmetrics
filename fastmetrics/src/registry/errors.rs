use std::{borrow::Cow, fmt};

/// Represents errors that can occur when registering metrics.
#[non_exhaustive]
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum RegistryError {
    /// Namespace configuration is invalid.
    InvalidNamespace { namespace: Cow<'static, str>, reason: String },
    /// Subsystem name is invalid.
    InvalidSubsystemName { name: Cow<'static, str>, reason: String },
    /// Metric name is invalid.
    InvalidMetricName { name: Cow<'static, str>, reason: String },
    /// Help text is invalid.
    InvalidHelpText { name: Cow<'static, str>, help: Cow<'static, str>, reason: String },
    /// Unit is invalid.
    InvalidUnit { name: Cow<'static, str>, unit: Cow<'static, str>, reason: String },
    /// Label name is invalid.
    InvalidLabelName { name: Cow<'static, str>, reason: String },
    /// The registered metric already exists in the registry.
    AlreadyExists { name: Cow<'static, str> },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNamespace { namespace, reason } => {
                write!(f, "namespace '{namespace}' is invalid: {reason}")
            },
            Self::InvalidSubsystemName { name, reason } => {
                write!(f, "subsystem name '{name}' is invalid: {reason}")
            },
            Self::InvalidMetricName { name, reason } => {
                write!(f, "metric name '{name}' is invalid: {reason}")
            },
            Self::InvalidHelpText { name, help, reason } => {
                write!(f, "help text '{help}' for metric '{name}' is invalid: {reason}")
            },
            Self::InvalidUnit { name, unit, reason } => {
                write!(f, "unit '{unit}' for metric '{name}' is invalid: {reason}")
            },
            Self::InvalidLabelName { name, reason } => {
                write!(f, "label name '{name}' is invalid: {reason}")
            },
            Self::AlreadyExists { name } => {
                write!(f, "metric '{name}' to be registered already exists in the registry")
            },
        }
    }
}

impl std::error::Error for RegistryError {}
