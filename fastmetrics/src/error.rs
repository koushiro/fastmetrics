//! Errors that are returned by FastMetrics.

use std::{
    backtrace::{Backtrace, BacktraceStatus},
    borrow::Cow,
    error::Error as StdError,
    fmt,
};

/// Result that is a wrapper of `Result<T, fastmetrics::Error>`
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// All kinds of error.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// We don't know what happened here, and no actions other than just returning it back.
    Unexpected,
    /// The operation is not supported.
    Unsupported,
    /// The definition is invalid.
    Invalid,
    /// The operation is duplicated.
    Duplicated,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unexpected => f.write_str("Unexpected"),
            Self::Unsupported => f.write_str("Unsupported"),
            Self::Invalid => f.write_str("Invalid"),
            Self::Duplicated => f.write_str("Duplicated"),
        }
    }
}

impl ErrorKind {
    /// Capturing a backtrace can be a quite expensive runtime operation.
    /// For some kinds of errors, backtrace is not useful and we can skip it.
    fn enable_backtrace(&self) -> bool {
        matches!(self, ErrorKind::Unexpected)
    }
}

/// The error struct returned by all fastmetrics functions.
pub struct Error {
    kind: ErrorKind,
    message: Cow<'static, str>,
    context: Vec<(&'static str, String)>,

    source: Option<anyhow::Error>,
    backtrace: Option<Box<Backtrace>>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return f
                .debug_struct("Error")
                .field("kind", &self.kind)
                .field("message", &self.message)
                .field("context", &self.context)
                .field("source", &self.source)
                .finish();
        }

        write!(f, "{}", self.kind)?;
        if !self.message.is_empty() {
            write!(f, " => {}", self.message)?;
        }
        writeln!(f)?;

        if !self.context.is_empty() {
            writeln!(f)?;
            writeln!(f, "Context:")?;
            for (k, v) in self.context.iter() {
                writeln!(f, "   {k}: {v}")?;
            }
        }

        if let Some(source) = &self.source {
            writeln!(f)?;
            writeln!(f, "Source:")?;
            writeln!(f, "   {source:#}")?;
        }

        if let Some(backtrace) = &self.backtrace {
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            writeln!(f, "{backtrace}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if !self.context.is_empty() {
            write!(f, ", context: {{ ")?;
            write!(
                f,
                "{}",
                self.context
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
            write!(f, " }}")?;
        }

        if !self.message.is_empty() {
            write!(f, " => {}", self.message)?;
        }

        if let Some(source) = &self.source {
            write!(f, ", source: {source}")?;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|v| v.as_ref())
    }
}

impl Error {
    /// Create a new [`Error`] with error kind and message.
    pub fn new(kind: ErrorKind, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            message: message.into(),
            context: Vec::new(),

            source: None,
            backtrace: kind
                .enable_backtrace()
                // `Backtrace::capture()` will check if backtrace has been enabled internally.
                // It's zero cost if backtrace is disabled.
                .then(Backtrace::capture)
                // We only keep captured backtrace to avoid an extra box.
                .filter(|bt| bt.status() == BacktraceStatus::Captured)
                .map(Box::new),
        }
    }

    /// Create a new unexpected [`Error`] with message.
    pub fn unexpected(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ErrorKind::Unexpected, message)
    }

    /// Create a new unsupported [`Error`] with message.
    pub fn unsupported(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ErrorKind::Unsupported, message)
    }

    /// Create a new invalid [`Error`] with message.
    pub fn invalid(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ErrorKind::Invalid, message)
    }

    /// Create a new duplicated [`Error`] with message.
    pub fn duplicated(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ErrorKind::Duplicated, message)
    }

    /// Add more context in error.
    pub fn with_context(mut self, key: &'static str, value: impl ToString) -> Self {
        self.context.push((key, value.to_string()));
        self
    }

    /// Set source for error.
    ///
    /// # Notes
    ///
    /// If the source has been set, we will raise a panic here.
    pub fn set_source(mut self, src: impl Into<anyhow::Error>) -> Self {
        debug_assert!(self.source.is_none(), "the source error has been set");
        self.source = Some(src.into());
        self
    }

    /// Returns the kind of the error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the message of the error.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::unexpected("failed to encode text").set_source(err)
    }
}
