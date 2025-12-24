//! [Open Metrics Histogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#histogram) metric type.
//!
//! See [`Histogram`] for more details.

use std::{
    fmt::{self, Debug},
    sync::{Arc, atomic::*},
    time::Duration,
};

pub use crate::raw::bucket::*;
use crate::{
    encoder::{EncodeMetric, MetricEncoder},
    error::Result,
    raw::{Atomic, MetricLabelSet, MetricType, TypedMetric},
};

/// Open Metrics [`Histogram`] metric, which samples observations and counts them in configurable
/// buckets.
///
/// # Example
///
/// ```rust
/// # use std::time::SystemTime;
/// #
/// # use fastmetrics::metrics::histogram::{linear_buckets, Histogram};
/// #
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
/// let created = SystemTime::UNIX_EPOCH
///     .elapsed()
///     .expect("UNIX timestamp when the histogram was created");
/// let hist = Histogram::with_created(linear_buckets(1.0, 1.0, 10), created);
/// assert!(hist.created().is_some());
/// ```
#[derive(Clone)]
pub struct Histogram {
    inner: Arc<HistogramInner>,
    // UNIX timestamp
    created: Option<Duration>,
}

struct HistogramInner {
    buckets: Vec<BucketCell>,
    count: AtomicU64,
    sum: AtomicU64,
}

struct BucketCell {
    upper_bound: f64,
    count: AtomicU64,
}

impl BucketCell {
    fn new(upper_bound: f64) -> Self {
        Self { upper_bound, count: AtomicU64::new(0) }
    }

    fn inc(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    fn load(&self) -> Bucket {
        Bucket::new(self.upper_bound, self.count.load(Ordering::Relaxed))
    }
}

impl HistogramInner {
    fn from_bounds(buckets: impl IntoIterator<Item = f64>) -> Self {
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
        let buckets = upper_bounds.into_iter().map(BucketCell::new).collect::<Vec<_>>();

        Self { buckets, count: AtomicU64::new(0), sum: AtomicU64::new(0f64.to_bits()) }
    }

    fn bucket_index(&self, value: f64) -> usize {
        self.buckets.partition_point(|bucket| bucket.upper_bound < value)
    }

    fn snapshot(&self) -> HistogramSnapshot {
        let buckets = self.buckets.iter().map(BucketCell::load).collect();
        let count = self.count.load(Ordering::Relaxed);
        let sum = self.sum.get();
        HistogramSnapshot { buckets, count, sum }
    }
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
        let created = self.created();
        self.with_snapshot(|snapshot| {
            f.debug_struct("Histogram")
                .field("buckets", &snapshot.buckets())
                .field("sum", &snapshot.sum())
                .field("count", &snapshot.count())
                .field("created", &created)
                .finish()
        })
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
        Self { inner: Arc::new(HistogramInner::from_bounds(buckets)), created: None }
    }

    /// Creates a [`Histogram`] with a `created` timestamp.
    pub fn with_created(buckets: impl IntoIterator<Item = f64>, created: Duration) -> Self {
        Self { inner: Arc::new(HistogramInner::from_bounds(buckets)), created: Some(created) }
    }

    /// Observes a value, incrementing the appropriate buckets.
    pub fn observe(&self, value: f64) {
        // value MUST NOT be NaN or negative
        if value.is_nan() || value.is_sign_negative() {
            return;
        }

        // increment the count and add the value into the sum
        self.inner.count.fetch_add(1, Ordering::Relaxed);
        self.inner.sum.inc_by(value);

        // only increment the count of the found bucket
        let idx = self.inner.bucket_index(value);
        self.inner.buckets[idx].inc();
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
    /// #
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
        let snapshot = self.inner.snapshot();
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

impl MetricLabelSet for Histogram {
    type LabelSet = ();
}

impl EncodeMetric for Histogram {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
        let created = self.created();
        self.with_snapshot(|s| {
            let buckets = s.buckets();
            let exemplars = None;
            encoder.encode_histogram(buckets, exemplars, s.count(), s.sum(), created)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::check_text_encoding;

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

        let created = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("UNIX timestamp when the histogram was created");
        let hist = Histogram::with_created(vec![1.0, 2.0], created);
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

    #[test]
    fn test_text_encoding() {
        check_text_encoding(
            |registry| {
                let hist = Histogram::new(exponential_buckets(1.0, 2.0, 5));
                registry.register("my_histogram", "My histogram help", hist.clone()).unwrap();
                for i in 1..=100 {
                    hist.observe(i as f64);
                }
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_histogram histogram
                    # HELP my_histogram My histogram help
                    my_histogram_bucket{le="1.0"} 1
                    my_histogram_bucket{le="2.0"} 2
                    my_histogram_bucket{le="4.0"} 4
                    my_histogram_bucket{le="8.0"} 8
                    my_histogram_bucket{le="16.0"} 16
                    my_histogram_bucket{le="+Inf"} 100
                    my_histogram_count 100
                    my_histogram_sum 5050.0
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );
    }
}
