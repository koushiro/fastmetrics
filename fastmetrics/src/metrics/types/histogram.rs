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
/// buckets.
///
/// # Example
///
/// ```rust
/// use fastmetrics::metrics::histogram::{linear_buckets, Histogram};
/// // Create a histogram with custom bucket boundaries
/// let hist = Histogram::new(linear_buckets(1.0, 1.0, 10));
///
/// // Observe some values
/// hist.observe(0.5);  // Falls into ≤1.0 bucket
/// hist.observe(1.5);  // Falls into ≤2.0 bucket
/// hist.observe(3.0);  // Falls into ≤3.0 bucket
/// hist.observe(20.0); // Falls into +Inf bucket
///
/// // Check snapshot
/// hist.with_snapshot(|s| {
///     // Get bucket counts
///     let buckets = s.buckets();
///     assert_eq!(buckets[0].upper_bound(), 1.0);
///     assert_eq!(buckets[0].count(), 1);  // One value ≤1.0
///     assert_eq!(buckets[1].upper_bound(), 2.0);
///     assert_eq!(buckets[1].count(), 1);  // One value ≤2.0
///     assert_eq!(buckets[2].upper_bound(), 3.0);
///     assert_eq!(buckets[2].count(), 1);  // One value ≤5.0
///     assert_eq!(buckets[10].upper_bound(), f64::INFINITY);
///     assert_eq!(buckets[10].count(), 1);  // One value in +Inf bucket
///     // Get count and sum statistics
///     assert_eq!(s.count(), 4);      // Total number of observations
///     assert_eq!(s.sum(), 25.0);     // Sum of all observed values
/// });
///
/// // Create a histogram with created timestamp
/// let hist = Histogram::with_created(linear_buckets(1.0, 1.0, 10));
/// assert!(hist.created().is_some());
/// ```
#[derive(Clone)]
pub struct Histogram {
    inner: Arc<RwLock<HistogramSnapshot>>,
    // UNIX timestamp
    created: Option<Duration>,
}

/// A snapshot of a [`Histogram`] at a point in time.
#[derive(Clone)]
pub struct HistogramSnapshot {
    buckets: Vec<Bucket>,
    count: u64,
    sum: f64,
}

impl HistogramSnapshot {
    /// Gets the current `bucket` counts.
    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    /// Gets the current `count` of all observations.
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Gets the current `sum` of all observed values.
    pub const fn sum(&self) -> f64 {
        self.sum
    }
}

impl Debug for Histogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let snapshot = self.inner.read();
        let created = self.created();

        f.debug_struct("Histogram")
            .field("buckets", &snapshot.buckets())
            .field("sum", &snapshot.sum())
            .field("count", &snapshot.count())
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
        // filter the NaN and negative bound
        let mut upper_bounds = buckets
            .into_iter()
            .filter(|upper_bound| !upper_bound.is_nan() && upper_bound.is_sign_positive())
            .collect::<Vec<_>>();
        // sort and dedup the bounds
        upper_bounds.sort_by(|a, b| a.partial_cmp(b).expect("upper_bound must not be NaN"));
        upper_bounds.dedup();

        // ensure +Inf bucket is included
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
            inner: Arc::new(RwLock::new(HistogramSnapshot { buckets, sum: 0f64, count: 0 })),
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
        // value MUST NOT be NaN or negative
        if value.is_nan() || value.is_sign_negative() {
            return;
        }

        let mut inner = self.inner.write();
        // increment the count and add the value into the sum
        inner.count += 1;
        inner.sum += value;

        // only increment the count of the found bucket
        let idx = inner.buckets.partition_point(|bucket| bucket.upper_bound() < value);
        inner.buckets[idx].inc();
    }

    /// Provides temporary access to a snapshot of the histogram's current state.
    ///
    /// # Arguments
    ///
    /// * `func` - A closure that receives a reference to the [`HistogramSnapshot`].
    ///
    /// # Returns
    ///
    /// The value returned by the provided closure.
    ///
    /// # Example
    ///
    /// ```
    /// # use fastmetrics::metrics::histogram::{Histogram, linear_buckets};
    /// let hist = Histogram::new(linear_buckets(1.0, 1.0, 3));
    /// hist.observe(2.5);
    ///
    /// hist.with_snapshot(|s| {
    ///     assert_eq!(s.count(), 1);
    ///     assert_eq!(s.sum(), 2.5);
    /// });
    /// ```
    pub fn with_snapshot<F, R>(&self, func: F) -> R
    where
        F: FnOnce(&HistogramSnapshot) -> R,
    {
        let snapshot = self.inner.read();
        func(&snapshot)
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
        hist.with_snapshot(|s| {
            let buckets = s.buckets();
            assert_eq!(buckets.len(), DEFAULT_BUCKETS.len() + 1); // Including +Inf bucket
            assert_eq!(s.count(), 0);
            assert_eq!(s.sum(), 0.0);
        });

        assert!(hist.created().is_none());

        let bounds = vec![1.0, 2.0, 5.0];
        let hist = Histogram::new(bounds);
        hist.with_snapshot(|s| {
            let buckets = s.buckets();
            assert_eq!(buckets.len(), 4); // Including +Inf bucket
            assert_eq!(buckets[0].upper_bound(), 1.0);
            assert_eq!(buckets[1].upper_bound(), 2.0);
            assert_eq!(buckets[2].upper_bound(), 5.0);
            assert_eq!(buckets[3].upper_bound(), f64::INFINITY);
        });

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

        hist.with_snapshot(|s| {
            let buckets = s.buckets();
            assert_eq!(buckets[0].count(), 1); // ≤1.0
            assert_eq!(buckets[1].count(), 1); // ≤2.0
            assert_eq!(buckets[2].count(), 1); // ≤5.0
            assert_eq!(buckets[3].count(), 1); // +Inf
            assert_eq!(s.count(), 4);
            assert_eq!(s.sum(), 11.0);
        });
    }

    #[test]
    fn test_histogram_invalid_observations() {
        let hist = Histogram::default();

        hist.observe(-1.0); // Negative value
        hist.observe(f64::NAN); // NaN value

        hist.with_snapshot(|s| {
            assert_eq!(s.count(), 0);
            assert_eq!(s.sum(), 0.0);
        });
    }

    #[test]
    fn test_histogram_thread_safe() {
        let hist = Histogram::new(vec![1.0, 2.0, 5.0]);
        let clone = hist.clone();

        let handle = std::thread::spawn(move || {
            for i in 1..=100 {
                clone.observe(i as f64);
            }
        });

        for i in 1..=100 {
            hist.observe(i as f64);
        }

        handle.join().unwrap();

        hist.with_snapshot(|s| {
            assert_eq!(s.count(), 200);
            assert_eq!(s.sum(), 10100.0);
        });
    }
}
