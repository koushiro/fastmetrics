//! Shared internal building blocks for "lazy" metric types.
//!
//! It exists to reduce duplication between `LazyGauge` and `LazyCounter`
//! (and any future lazy metric types) by providing a common source abstraction.
//!
//! Note: the public API intentionally keeps `LazyGauge` / `LazyCounter` focused and does not
//! expose these internal traits.

use std::sync::Arc;

/// Internal source of values for "lazy" metrics.
///
/// This trait is crate-private so other internal modules (e.g. `metrics::lazy_group`) can provide
/// alternative implementations (such as a scrape-scoped cached source) without exposing extra
/// public types.
///
/// Implementations must be thread-safe (`Send + Sync`) because metrics may be encoded from multiple
/// threads depending on the exporter design.
///
/// Implementations should be cheap to call; the encoder may call this during scrapes to obtain the
/// current value.
pub(crate) trait LazySource<T>: Send + Sync {
    /// Returns the current value by evaluating the underlying source.
    fn load(&self) -> T;
}

/// A simple [`LazySource`] implementation backed by a closure.
///
/// This is used by `LazyGauge::new(...)` / `LazyCounter::new(...)` when the user provides a
/// fetcher.
pub(crate) struct PlainLazySource<T> {
    fetch: Arc<dyn Fn() -> T + Send + Sync>,
}

impl<T> PlainLazySource<T> {
    #[inline]
    pub(crate) fn new(fetch: Arc<dyn Fn() -> T + Send + Sync>) -> Self {
        Self { fetch }
    }
}

impl<T> LazySource<T> for PlainLazySource<T> {
    #[inline]
    fn load(&self) -> T {
        (self.fetch.as_ref())()
    }
}
