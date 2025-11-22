use std::{borrow::Cow, fmt};

use crate::raw::MetricType;

/// Represents errors that can occur when registering metrics.
#[non_exhaustive]
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum RegistryError {
    /// The registered metric already exists in the registry
    AlreadyExists { name: Cow<'static, str> },
    /// MetricFamilies of type StateSet and Info must have an empty Unit string
    MustHaveAnEmptyUnitString { name: Cow<'static, str> },
    /// Metric unit format must be lowercase
    OtherUnitFormatMustBeLowercase { unit: Cow<'static, str> },
    /// Name format is invalid
    InvalidNameFormat { name: Cow<'static, str> },
    /// Reserved label name
    ReservedLabelName { name: Cow<'static, str>, ty: MetricType },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { name } => {
                write!(f, "The metric '{name}' to be registered already exists in the registry")
            },
            Self::MustHaveAnEmptyUnitString { name } => {
                write!(f, "The type of metric '{name}' must have an empty unit string")
            },
            Self::OtherUnitFormatMustBeLowercase { unit } => {
                write!(f, "The format of unit '{unit}' must be lowercase")
            },
            Self::InvalidNameFormat { name } => {
                write!(f, "The name '{name}' should be snake_case")
            },
            Self::ReservedLabelName { name, ty } => {
                write!(f, "The label name '{name}' is reserved for '{ty:?}' type")
            },
        }
    }
}

impl std::error::Error for RegistryError {}
