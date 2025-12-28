//! Helpers for lazy metric fetchers shared across metric types.
//!
//! Provides a `Fetch` trait that captures the output type of zero-arg callables
//! and a convenience alias `OutputOf<F>` to refer to that output.

/// Extracts the output type of a zero-argument callable used as a lazy metric fetcher.
pub trait Fetch {
    /// The value produced by the fetcher.
    type Output;
}

impl<F, O> Fetch for F
where
    F: Fn() -> O,
{
    type Output = O;
}

/// Convenience alias for the output type of a fetcher.
pub type OutputOf<F> = <F as Fetch>::Output;
