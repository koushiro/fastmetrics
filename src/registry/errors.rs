use std::fmt;

/// Represents errors that can occur when registering metrics.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum RegistryError {
    AlreadyExists,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists => {
                f.write_str("The metric being registered already exists in the registry")
            },
        }
    }
}

impl std::error::Error for RegistryError {}
