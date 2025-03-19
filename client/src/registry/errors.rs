use std::fmt;

/// Represents errors that can occur when registering metrics.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum RegistryError {
    /// The registered metric already exists in the registry
    AlreadyExists,
    /// MetricFamilies of type StateSet and Info must have an empty Unit string
    MustHaveAnEmptyUnitString,
    /// Metric name format is invalid
    InvalidNameFormat,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists => {
                f.write_str("The registered metric already exists in the registry")
            },
            Self::MustHaveAnEmptyUnitString => {
                f.write_str("The metric type must have an empty unit string")
            },
            Self::InvalidNameFormat => f.write_str("The name should be snake_case"),
        }
    }
}

impl std::error::Error for RegistryError {}
