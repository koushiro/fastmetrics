//! [Open Metrics GaugeHistogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#gaugehistogram) metric type.
//!
//! See [`GaugeHistogram`] for more details.

use std::{
    fmt::{self, Debug},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

pub use crate::raw::bucket::*;
use crate::{
    encoder::{EncodeMetric, MetricEncoder},
    raw::{Atomic, MetricLabelSet, MetricType, TypedMetric},
};

/// Open Metrics [`GaugeHistogram`] metric, which samples observations and counts them in
/// configurable buckets.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::gauge_histogram::{linear_buckets, GaugeHistogram};
/// // Create a gauge histogram with custom bucket boundaries
/// let hist = GaugeHistogram::new([-273.15, -200.0, -100.0, 0.0, 100.0, 200.0]);
///
/// // Observe some values
/// hist.observe(-250.0);   // Falls into ≤-200 bucket
/// hist.observe(0.0);      // Falls into ≤0.0 bucket
/// hist.observe(100.0);    // Falls into ≤100.0 bucket
/// hist.observe(1000.0);   // Falls into +Inf bucket
///
/// // Check snapshot
/// hist.with_snapshot(|s| {
///     // Get bucket counts
///     let buckets = s.buckets();
///     assert_eq!(buckets[1].upper_bound(), -200.0);
///     assert_eq!(buckets[1].count(), 1);  // One value ≤-200.0
///     assert_eq!(buckets[3].upper_bound(), 0.0);
///     assert_eq!(buckets[3].count(), 1);  // One value ≤0.0
///     assert_eq!(buckets[4].upper_bound(), 100.0);
///     assert_eq!(buckets[4].count(), 1);  // One value ≤100.0
///     assert_eq!(buckets[6].upper_bound(), f64::INFINITY);
///     assert_eq!(buckets[6].count(), 1);  // One value in +Inf bucket
///     // Get gcount and gsum statistics
///     assert_eq!(s.gcount(), 4);       // Total number of observations
///     assert_eq!(s.gsum(), 850.0);     // Sum of all observed values
/// });
/// ```
#[derive(Clone)]
pub struct GaugeHistogram {
    inner: Arc<GaugeHistogramInner>,
}

struct GaugeHistogramInner {
    buckets: Vec<BucketCell>,
    gsum: AtomicU64,
    gcount: AtomicU64,
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

impl GaugeHistogramInner {
    fn from_bounds(buckets: impl IntoIterator<Item = f64>) -> Self {
        // filter the NaN bound
        let mut upper_bounds = buckets
            .into_iter()
            .filter(|upper_bound| !upper_bound.is_nan())
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

        Self { buckets, gcount: AtomicU64::new(0), gsum: AtomicU64::new(0f64.to_bits()) }
    }

    fn bucket_index(&self, value: f64) -> usize {
        self.buckets.partition_point(|bucket| bucket.upper_bound < value)
    }

    fn snapshot(&self) -> GaugeHistogramSnapshot {
        let buckets = self.buckets.iter().map(BucketCell::load).collect();
        let gcount = self.gcount.load(Ordering::Relaxed);
        let gsum = self.gsum.get();
        GaugeHistogramSnapshot { buckets, gcount, gsum }
    }
}

/// A snapshot of a [`GaugeHistogram`] at a point in time.
#[derive(Clone)]
pub struct GaugeHistogramSnapshot {
    buckets: Vec<Bucket>,
    gsum: f64,
    gcount: u64,
}

impl GaugeHistogramSnapshot {
    /// Gets the current `bucket` counts.
    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    /// Gets the current `gcount` of all observations.
    pub const fn gcount(&self) -> u64 {
        self.gcount
    }

    /// Gets the current `gsum` of all observed values.
    pub const fn gsum(&self) -> f64 {
        self.gsum
    }
}

impl Debug for GaugeHistogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with_snapshot(|snapshot| {
            f.debug_struct("GaugeHistogram")
                .field("buckets", &snapshot.buckets())
                .field("gcount", &snapshot.gcount())
                .field("gsum", &snapshot.gsum())
                .finish()
        })
    }
}

impl Default for GaugeHistogram {
    fn default() -> Self {
        Self::new(DEFAULT_BUCKETS)
    }
}

impl GaugeHistogram {
    /// Creates a new [`GaugeHistogram`] with the given bucket boundaries.
    pub fn new(buckets: impl IntoIterator<Item = f64>) -> Self {
        Self { inner: Arc::new(GaugeHistogramInner::from_bounds(buckets)) }
    }

    /// Observes a value, incrementing the appropriate buckets.
    pub fn observe(&self, value: f64) {
        // value MUST NOT be NaN
        if value.is_nan() {
            return;
        }

        // increment the gcount and add the value into the gsum
        self.inner.gcount.fetch_add(1, Ordering::Relaxed);
        self.inner.gsum.inc_by(value);

        // only increment the count of the found bucket
        let idx = self.inner.bucket_index(value);
        self.inner.buckets[idx].inc();
    }

    /// Provides temporary access to a snapshot of the gauge histogram's current state.
    ///
    /// # Parameters
    ///
    /// * `func` - A closure that receives a reference to the [`GaugeHistogramSnapshot`].
    ///
    /// # Returns
    ///
    /// The value returned by the provided closure
    ///
    /// # Example
    ///
    /// ```
    /// # use fastmetrics::metrics::gauge_histogram::GaugeHistogram;
    /// let histogram = GaugeHistogram::default();
    /// histogram.observe(42.0);
    ///
    /// histogram.with_snapshot(|s| {
    ///     assert_eq!(s.gcount(), 1);
    ///     assert_eq!(s.gsum(), 42.0);
    /// });
    /// ```
    pub fn with_snapshot<F, R>(&self, func: F) -> R
    where
        F: FnOnce(&GaugeHistogramSnapshot) -> R,
    {
        let snapshot = self.inner.snapshot();
        func(&snapshot)
    }
}

impl TypedMetric for GaugeHistogram {
    const TYPE: MetricType = MetricType::GaugeHistogram;
}

impl MetricLabelSet for GaugeHistogram {
    type LabelSet = ();
}

impl EncodeMetric for GaugeHistogram {
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        self.with_snapshot(|s| {
            let buckets = s.buckets();
            let exemplars = None;
            encoder.encode_gauge_histogram(buckets, exemplars, s.gcount(), s.gsum())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::check_text_encoding;

    #[test]
    fn test_gauge_histogram_initialization() {
        let hist = GaugeHistogram::default();
        hist.with_snapshot(|s| {
            let buckets = s.buckets();
            assert_eq!(buckets.len(), DEFAULT_BUCKETS.len() + 1); // Including +Inf bucket
            assert_eq!(s.gcount(), 0);
            assert_eq!(s.gsum(), 0.0);
        });

        let bounds = vec![1.0, 2.0, 5.0];
        let hist = GaugeHistogram::new(bounds);
        hist.with_snapshot(|s| {
            let buckets = s.buckets();
            assert_eq!(buckets.len(), 4); // Including +Inf bucket
            assert_eq!(buckets[0].upper_bound(), 1.0);
            assert_eq!(buckets[1].upper_bound(), 2.0);
            assert_eq!(buckets[2].upper_bound(), 5.0);
            assert_eq!(buckets[3].upper_bound(), f64::INFINITY);
        });
    }

    #[test]
    fn test_gauge_histogram_observe() {
        let hist = GaugeHistogram::new(vec![-273.15, -200.0, -100.0, 0.0, 100.0, 200.0]);

        hist.observe(-250.0);
        hist.observe(0.0);
        hist.observe(100.0);
        hist.observe(1000.0);

        hist.with_snapshot(|s| {
            let buckets = s.buckets();
            assert_eq!(buckets[1].count(), 1); // ≤-200.0
            assert_eq!(buckets[3].count(), 1); // ≤0.0
            assert_eq!(buckets[4].count(), 1); // ≤100.0
            assert_eq!(buckets[6].count(), 1); // +Inf
            assert_eq!(s.gcount(), 4);
            assert_eq!(s.gsum(), 850.0);
        });
    }

    #[test]
    fn test_gauge_histogram_invalid_observations() {
        let hist = GaugeHistogram::default();

        hist.observe(-1.0); // Negative value, valid
        hist.observe(f64::NAN); // NaN value, invalid

        hist.with_snapshot(|s| {
            assert_eq!(s.gcount(), 1);
            assert_eq!(s.gsum(), -1.0);
        });
    }

    #[test]
    fn test_gauge_histogram_thread_safe() {
        let hist = GaugeHistogram::new(vec![-273.15, -200.0, -100.0, 0.0, 100.0, 200.0]);
        let clone = hist.clone();

        let handle = std::thread::spawn(move || {
            for i in -100..0 {
                clone.observe(i as f64);
            }
            for i in 1..=100 {
                clone.observe(i as f64);
            }
        });

        for i in -100..0 {
            hist.observe(i as f64);
        }
        for i in 1..=100 {
            hist.observe(i as f64);
        }

        handle.join().unwrap();

        hist.with_snapshot(|s| {
            assert_eq!(s.gcount(), 400);
            assert_eq!(s.gsum(), 0.0);
        });
    }

    #[test]
    fn test_text_encoding() {
        check_text_encoding(
            |registry| {
                let hist = GaugeHistogram::new(exponential_buckets(1.0, 2.0, 5));
                registry
                    .register("my_histogram", "My gauge histogram help", hist.clone())
                    .unwrap();
                for i in 1..=100 {
                    hist.observe(i as f64);
                }
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE my_histogram gaugehistogram
                    # HELP my_histogram My gauge histogram help
                    my_histogram_bucket{le="1.0"} 1
                    my_histogram_bucket{le="2.0"} 2
                    my_histogram_bucket{le="4.0"} 4
                    my_histogram_bucket{le="8.0"} 8
                    my_histogram_bucket{le="16.0"} 16
                    my_histogram_bucket{le="+Inf"} 100
                    my_histogram_gcount 100
                    my_histogram_gsum 5050.0
                    # EOF
                "#};
                assert_eq!(output, expected);
            },
        );
    }
}
