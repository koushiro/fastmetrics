use std::fmt;

/// TODO: doc
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum RegistryError {
    AlreadyExists,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists => write!(f, "Metric already exists"),
        }
    }
}

impl std::error::Error for RegistryError {}
