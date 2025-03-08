//! [Open Metrics Histogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#histogram) metric type.
//!
//! See [`Histogram`] for more details.

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
///
/// # Example
///
/// ```rust
/// use openmetrics_client::metrics::histogram::{linear_buckets, Histogram};
/// // Create a histogram with custom bucket boundaries
/// let hist = Histogram::new(linear_buckets(1.0, 1.0, 10));
///
/// // Observe some values
/// hist.observe(0.5);  // Falls into ≤1.0 bucket
/// hist.observe(1.5);  // Falls into ≤2.0 bucket
/// hist.observe(3.0);  // Falls into ≤3.0 bucket
/// hist.observe(20.0); // Falls into +Inf bucket
///
/// // Get bucket counts
/// let buckets = hist.buckets();
/// assert_eq!(buckets[0].upper_bound(), 1.0);
/// assert_eq!(buckets[0].count(), 1);  // One value ≤1.0
/// assert_eq!(buckets[1].upper_bound(), 2.0);
/// assert_eq!(buckets[1].count(), 1);  // One value ≤2.0
/// assert_eq!(buckets[2].upper_bound(), 3.0);
/// assert_eq!(buckets[2].count(), 1);  // One value ≤5.0
/// assert_eq!(buckets[10].upper_bound(), f64::INFINITY);
/// assert_eq!(buckets[10].count(), 1);  // One value in +Inf bucket
///
/// // Get count and sum statistics
/// assert_eq!(hist.count(), 4);      // Total number of observations
/// assert_eq!(hist.sum(), 25.0);     // Sum of all observed values
///
/// // Create a histogram with created timestamp
/// let hist = Histogram::with_created(linear_buckets(1.0, 1.0, 10));
/// assert!(hist.created().is_some());
/// ```
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
        let inner = self.inner.read();
        let buckets = &inner.buckets;
        let sum = inner.sum;
        let count = inner.count;
        let created = self.created();

        f.debug_struct("Histogram")
            .field("buckets", buckets)
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
        // Filter the NaN and negative bound
        let mut upper_bounds = buckets
            .into_iter()
            .filter(|upper_bound| !upper_bound.is_nan() && upper_bound.is_sign_positive())
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
        let idx = inner.buckets.partition_point(|bucket| bucket.upper_bound() < value);
        inner.buckets[idx].inc();
    }

    /// Gets the current `bucket` counts.
    pub fn buckets(&self) -> Vec<Bucket> {
        self.inner.read().buckets.clone()
    }

    /// Gets the current `sum` of all observed values.
    pub fn sum(&self) -> f64 {
        self.inner.read().sum
    }

    /// Gets the current `count` of all observations.
    pub fn count(&self) -> u64 {
        self.inner.read().count
    }

    /// Gets the optional `created` value of the [`Histogram`].
    pub const fn created(&self) -> Option<Duration> {
        self.created
    }
}

impl TypedMetric for Histogram {
    const TYPE: MetricType = MetricType::Histogram;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_initialization() {
        let hist = Histogram::default();
        let buckets = hist.buckets();
        assert_eq!(buckets.len(), DEFAULT_BUCKETS.len() + 1); // Including +Inf bucket
        assert_eq!(hist.sum(), 0.0);
        assert_eq!(hist.count(), 0);
        assert!(hist.created().is_none());

        let bounds = vec![1.0, 2.0, 5.0];
        let hist = Histogram::new(bounds);
        let buckets = hist.buckets();
        assert_eq!(buckets.len(), 4); // Including +Inf bucket
        assert_eq!(buckets[0].upper_bound(), 1.0);
        assert_eq!(buckets[1].upper_bound(), 2.0);
        assert_eq!(buckets[2].upper_bound(), 5.0);
        assert_eq!(buckets[3].upper_bound(), f64::INFINITY);

        let hist = Histogram::with_created(vec![1.0, 2.0]);
        assert!(hist.created().is_some());
    }

    #[test]
    fn test_histogram_observe() {
        let hist = Histogram::new(vec![1.0, 2.0, 5.0]);

        hist.observe(1.5);
        hist.observe(0.5);
        hist.observe(3.0);
        hist.observe(6.0);

        let buckets = hist.buckets();
        assert_eq!(buckets[0].count(), 1); // ≤1.0
        assert_eq!(buckets[1].count(), 1); // ≤2.0
        assert_eq!(buckets[2].count(), 1); // ≤5.0
        assert_eq!(buckets[3].count(), 1); // +Inf
        assert_eq!(hist.count(), 4);
        assert_eq!(hist.sum(), 11.0);
    }

    #[test]
    fn test_histogram_invalid_observations() {
        let hist = Histogram::default();

        hist.observe(-1.0); // Negative value
        hist.observe(f64::NAN); // NaN value

        assert_eq!(hist.count(), 0);
        assert_eq!(hist.sum(), 0.0);
    }

    #[test]
    fn test_histogram_thread_safe() {
        let hist = Histogram::new(vec![1.0, 2.0, 5.0]);
        let clone = hist.clone();

        let handle = std::thread::spawn(move || {
            for i in 0..1000 {
                clone.observe(i as f64);
            }
        });

        for i in 0..1000 {
            hist.observe(i as f64);
        }

        handle.join().unwrap();
        assert_eq!(hist.count(), 2000);
    }
}
