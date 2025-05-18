use std::{borrow::Cow, fmt};

/// Represents errors that can occur when registering metrics.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum RegistryError {
    /// The registered metric already exists in the registry
    AlreadyExists {
        /// Metric name
        name: Cow<'static, str>,
    },
    /// MetricFamilies of type StateSet and Info must have an empty Unit string
    MustHaveAnEmptyUnitString {
        /// Metric name
        name: Cow<'static, str>,
    },
    /// Metric unit format must be lowercase
    OtherUnitFormatMustBeLowercase {
        /// Metric unit
        unit: Cow<'static, str>,
    },
    /// Metric name format is invalid
    InvalidNameFormat {
        /// Metric name
        name: Cow<'static, str>,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { name } => {
                write!(f, "The metric {name} to be registered already exists in the registry")
            },
            Self::MustHaveAnEmptyUnitString { name } => {
                write!(f, "The type of metric {name} must have an empty unit string")
            },
            Self::OtherUnitFormatMustBeLowercase { unit } => {
                write!(f, "The format of unit {unit} must be lowercase")
            },
            Self::InvalidNameFormat { name } => {
                write!(f, "The name of metric `{name}` should be snake_case")
            },
        }
    }
}

impl std::error::Error for RegistryError {}
