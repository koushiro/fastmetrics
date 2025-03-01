//! [Open Metrics Histogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#histogram) metric type.

use std::{
    fmt::{self, Debug},
    sync::Arc,
    time::{Duration, SystemTime},
};

use parking_lot::RwLock;

pub use crate::metrics::raw::bucket::*;
use crate::metrics::{MetricType, TypedMetric};

/// Open Metrics [`Histogram`] metric, which samples observations and counts them in configurable
/// buckets. This implementation uses f64 for the sum.
#[derive(Clone)]
pub struct Histogram {
    inner: Arc<RwLock<HistogramInner>>,
    // UNIX timestamp
    created: Option<Duration>,
}

struct HistogramInner {
    buckets: Vec<Bucket>,
    sum: f64,
    count: u64,
}

impl Debug for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (buckets, sum, count, created) = self.get();

        f.debug_struct("Histogram")
            .field("buckets", &buckets)
            .field("sum", &sum)
            .field("count", &count)
            .field("created", &created)
            .finish()
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new(DEFAULT_BUCKETS)
    }
}

impl Histogram {
    /// Creates a new [`Histogram`] with the given bucket boundaries.
    pub fn new(buckets: impl IntoIterator<Item = f64>) -> Self {
        // Filter the NaN bound
        let mut upper_bounds = buckets
            .into_iter()
            .filter(|upper_bound| !upper_bound.is_nan())
            .collect::<Vec<_>>();
        // Sort and dedup the bounds
        upper_bounds.sort_by(|a, b| a.partial_cmp(b).expect("upper_bound must not be NaN"));
        upper_bounds.dedup();

        // Ensure +Inf bucket is included
        match upper_bounds.last() {
            Some(last) if last.is_finite() => upper_bounds.push(f64::INFINITY),
            None => upper_bounds.push(f64::INFINITY),
            _ => { /* do nothing */ },
        }
        let buckets = upper_bounds
            .into_iter()
            .map(|upper_bound| Bucket::new(upper_bound, 0))
            .collect::<Vec<_>>();

        Self {
            inner: Arc::new(RwLock::new(HistogramInner { buckets, sum: 0f64, count: 0 })),
            created: None,
        }
    }

    /// Creates a [`Histogram`] with a `created` timestamp.
    pub fn with_created(buckets: impl IntoIterator<Item = f64>) -> Self {
        let mut this = Self::new(buckets);
        this.created = Some(
            SystemTime::UNIX_EPOCH
                .elapsed()
                .expect("UNIX timestamp when the histogram was created"),
        );
        this
    }

    /// Observes a value, incrementing the appropriate buckets.
    pub fn observe(&self, value: f64) {
        // Sum and bucket values MUST NOT be NaN or negative
        if value.is_nan() || value.is_sign_negative() {
            return;
        }

        let mut inner = self.inner.write();
        // Increment the count and add the value into the sum
        inner.count += 1;
        inner.sum += value;

        // Only increment the count of the found bucket
        let idx = inner
            .buckets
            .binary_search_by(|bucket| {
                bucket
                    .upper_bound()
                    .partial_cmp(&value)
                    .expect("upper_bound && value must not be NaN")
            })
            .expect("should be found");
        inner.buckets[idx].inc();
    }

    /// Gets the current bucket counts, sum, count, and optional created timestamp.
    pub fn get(&self) -> (Vec<Bucket>, f64, u64, Option<Duration>) {
        let buckets = self.buckets();
        let sum = self.sum();
        let count = self.count();
        (buckets, sum, count, self.created)
    }

    /// Gets the current bucket counts.
    pub fn buckets(&self) -> Vec<Bucket> {
        self.inner.read().buckets.clone()
    }

    /// Gets the current sum of all observed values.
    pub fn sum(&self) -> f64 {
        self.inner.read().sum
    }

    /// Gets the current count of observations.
    pub fn count(&self) -> u64 {
        self.inner.read().count
    }
}

impl TypedMetric for Histogram {
    const TYPE: MetricType = MetricType::Histogram;
}

/// A **constant** [`Histogram`], meaning it cannot be changed once created.
#[derive(Clone)]
pub struct ConstHistogram {
    buckets: Vec<Bucket>,
    sum: f64,
    count: u64,
    // UNIX timestamp
    created: Option<Duration>,
}

impl Debug for ConstHistogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (buckets, sum, count, created) = self.get();

        f.debug_struct("ConstHistogram")
            .field("buckets", &buckets)
            .field("sum", &sum)
            .field("count", &count)
            .field("created", &created)
            .finish()
    }
}

impl ConstHistogram {
    /// Creates a new [`ConstHistogram`] with the given bucket boundaries and counts.
    pub fn new(buckets: Vec<Bucket>, sum: f64, count: u64) -> Self {
        Self { buckets, sum, count, created: None }
    }

    /// Creates a [`ConstHistogram`] with a `created` timestamp.
    pub fn with_created(buckets: Vec<Bucket>, sum: f64, count: u64) -> Self {
        Self {
            buckets,
            sum,
            count,
            created: Some(
                SystemTime::UNIX_EPOCH
                    .elapsed()
                    .expect("UNIX timestamp when the histogram was created"),
            ),
        }
    }

    /// Gets the current bucket counts, sum, count, and optional created timestamp.
    pub fn get(&self) -> (&[Bucket], f64, u64, Option<Duration>) {
        (self.buckets(), self.sum(), self.count(), self.created)
    }

    /// Gets the current bucket counts.
    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    /// Gets the current sum of all observed values.
    pub const fn sum(&self) -> f64 {
        self.sum
    }

    /// Gets the current count of observations.
    pub const fn count(&self) -> u64 {
        self.count
    }
}

impl TypedMetric for ConstHistogram {
    const TYPE: MetricType = MetricType::Histogram;
}
