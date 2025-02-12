//! [Open Metrics Histogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#histogram) metric type.
//!
//! Not implemented yet.

pub use crate::metrics::raw::bucket::{exponential_buckets, linear_buckets, DEFAULT_BUCKETS};

/*
use std::{
    iter,
    sync::Arc,
    time::{Duration, SystemTime},
};

use parking_lot::RwLock;

use crate::metrics::bucket::Bucket;
pub use crate::metrics::bucket::{exponential_buckets, linear_buckets, DEFAULT_BUCKETS};

#[derive(Clone)]
pub struct Histogram {
    // TODO: use atomic
    inner: Arc<RwLock<HistogramInner>>,
    // UNIX timestamp
    created: Option<Duration>,
}

impl Histogram {
    /// Create a [`Histogram`].
    pub fn new(buckets: impl IntoIterator<Item = f64>) -> Self {
        Self { inner: Arc::new(RwLock::new(HistogramInner::new(buckets))), created: None }
    }

    /// Create a [`Histogram`] with a timestamp value called `created`.
    pub fn with_created(buckets: impl IntoIterator<Item = f64>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HistogramInner::new(buckets))),
            created: Some(
                SystemTime::UNIX_EPOCH
                    .elapsed()
                    .expect("UNIX timestamp when the counter was created"),
            ),
        }
    }

    /// Observe the given value.
    pub fn observe(&self, value: f64) {
        if value.is_nan() || value < 0.0 {
            return;
        }

        let mut inner = self.inner.write();
        inner.observe(value);
    }

    pub fn get(&self) -> HistogramSnapshot {
        let inner = self.inner.read();
        HistogramSnapshot { count: inner.count, sum: inner.sum, buckets: inner.buckets.clone() }
    }
}

struct HistogramInner {
    count: u64,
    sum: f64,
    buckets: Vec<Bucket>,
}

impl HistogramInner {
    fn new(buckets: impl IntoIterator<Item = f64>) -> Self {
        let mut upper_bounds = buckets
            .into_iter()
            .filter(|upper_bound| !upper_bound.is_nan())
            .collect::<Vec<_>>();
        upper_bounds.sort_by(|a, b| a.partial_cmp(b).expect("upper_bounds must not be NaN"));
        upper_bounds.dedup();

        Self {
            count: 0,
            sum: 0.0f64,
            buckets: upper_bounds
                .into_iter()
                .chain(iter::once(f64::INFINITY))
                .map(|upper_bound| Bucket { upper_bound, count: 0 })
                .collect::<Vec<_>>(),
        }
    }

    fn observe(&mut self, value: f64) {
        let idx = self
            .buckets
            .binary_search_by(|probe| {
                probe.upper_bound.partial_cmp(&value).expect("value must not be NaN")
            })
            .expect("should be found");

        self.sum += value;
        self.count += 1;
        self.buckets[idx].count += 1;
    }
}

pub struct HistogramSnapshot {
    count: u64,
    sum: f64,
    buckets: Vec<Bucket>,
}

impl HistogramSnapshot {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn sum(&self) -> f64 {
        self.sum
    }

    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }
}
*/
