//! Grouped lazy metrics.
//!
//! `LazyGroup` allows you to bind a set of scrape-time (lazy) metrics to a single shared sampling
//! operation *per scrape*.
//!
//! This is particularly useful for process/system metrics where one OS query can produce a full
//! snapshot struct, and multiple metrics should read different fields from that snapshot without
//! re-reading the OS for each metric.
//!
//! The sharing is scrape-scoped: it relies on the encoder entrypoint installing a scrape context
//! (e.g. `format::text::encode`).
//!
//! If no scrape context is active (e.g. calling `fetch()` directly), grouped metrics will fall back
//! to sampling on every call.
//!
//! # Note
//!
//! This module intentionally only provides the *grouping* primitive. The concrete metric types are
//! unified into `metrics::gauge::LazyGauge` and `metrics::counter::LazyCounter`.
//!
//! In other words, `LazyGroup::gauge(...)` returns a `LazyGauge`, and `LazyGroup::counter(...)`
//! returns a `LazyCounter`.
//!
//! The actual grouping behavior is implemented by those types. This keeps the API ergonomic and
//! avoids exposing extra "grouped" metric types.

use std::{sync::Arc, time::Duration};

use crate::{
    encoder::{EncodeCounterValue, EncodeGaugeValue},
    metrics::{
        counter::{CounterValue, LazyCounter},
        gauge::{GaugeValue, LazyGauge},
    },
};

mod id;
pub(crate) mod scrape_ctx;
mod source;

pub(crate) use self::id::LazyGroupId;

/// A group of lazily-evaluated metrics sharing a single sample per scrape.
///
/// Create a `LazyGroup` with a sampler function producing some snapshot `S`, then derive multiple
/// metrics from it via `gauge(...)` / `counter(...)`.
///
/// # Example
///
/// ```rust
/// use fastmetrics::metrics::lazy_group::LazyGroup;
///
/// #[derive(Clone, Copy)]
/// struct Sample {
///     a: u64,
///     b: i64,
/// }
///
/// let group = LazyGroup::new(|| Sample { a: 1, b: 2 });
///
/// let a = group.counter(|s| s.a);
/// let b = group.gauge(|s| s.b);
///
/// // register `a` and `b` as usual...
/// ```
pub struct LazyGroup<S> {
    pub(crate) id: LazyGroupId,
    pub(crate) sample: Arc<dyn Fn() -> S + Send + Sync>,
}

impl<S> Clone for LazyGroup<S> {
    fn clone(&self) -> Self {
        Self { id: self.id, sample: Arc::clone(&self.sample) }
    }
}

impl<S> LazyGroup<S>
where
    S: Send + Sync + 'static,
{
    /// Creates a new `LazyGroup` with the provided sampler.
    pub fn new(sample: impl Fn() -> S + Send + Sync + 'static) -> Self {
        let id = id::next_lazy_group_id();
        Self { id, sample: Arc::new(sample) }
    }

    /// Creates a lazy counter derived from the shared sample.
    ///
    /// The returned type is the standard [`LazyCounter`], with an internal grouped source
    /// so that all metrics derived from the same `LazyGroup` share one sample per scrape.
    pub fn counter<N, M>(&self, map: M) -> LazyCounter<N>
    where
        M: Fn(&S) -> N + Send + Sync + 'static,
        N: EncodeCounterValue + CounterValue + 'static,
    {
        source::counter_from_group(self.clone(), map, None)
    }

    /// Creates a lazy counter derived from the shared sample, with an explicit creation timestamp.
    pub fn counter_with_created<N, M>(&self, map: M, created: Duration) -> LazyCounter<N>
    where
        M: Fn(&S) -> N + Send + Sync + 'static,
        N: EncodeCounterValue + CounterValue + 'static,
    {
        source::counter_from_group(self.clone(), map, Some(created))
    }

    /// Creates a lazy gauge derived from the shared sample.
    ///
    /// The returned type is the standard [`LazyGauge`], with an internal grouped source
    /// so that all metrics derived from the same `LazyGroup` share one sample per scrape.
    pub fn gauge<N, M>(&self, map: M) -> LazyGauge<N>
    where
        M: Fn(&S) -> N + Send + Sync + 'static,
        N: EncodeGaugeValue + GaugeValue + 'static,
    {
        source::gauge_from_group(self.clone(), map)
    }
}
