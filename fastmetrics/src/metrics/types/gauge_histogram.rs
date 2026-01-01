//! [Open Metrics GaugeHistogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#gaugehistogram) metric type.
//!
//! See [`GaugeHistogram`] for more details.

use std::{
    fmt::{self, Debug},
    sync::Arc,
};

use crate::{
    encoder::{EncodeMetric, MetricEncoder},
    error::Result,
    metrics::internal::histogram::{BoundsFilter, HistogramCore},
    raw::{MetricLabelSet, MetricType, TypedMetric},
};
pub use crate::{metrics::internal::histogram::HistogramSnapshot, raw::bucket::*};

/// Open Metrics [`GaugeHistogram`] metric, which samples observations and counts them in
/// configurable buckets.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::gauge_histogram::{linear_buckets, GaugeHistogram};
/// #
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
///     // Get count and sum statistics
///     assert_eq!(s.count(), 4);       // Total number of observations
///     assert_eq!(s.sum(), 850.0);     // Sum of all observed values
/// });
/// ```
#[derive(Clone)]
pub struct GaugeHistogram {
    inner: Arc<HistogramCore>,
}

impl Debug for GaugeHistogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.with_snapshot(|snapshot| {
            f.debug_struct("GaugeHistogram")
                .field("buckets", &snapshot.buckets())
                .field("count", &snapshot.count())
                .field("sum", &snapshot.sum())
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
        Self { inner: Arc::new(HistogramCore::from_bounds(buckets, BoundsFilter::AllowNegative)) }
    }

    /// Observes a value, incrementing the appropriate buckets.
    pub fn observe(&self, value: f64) {
        // value MUST NOT be NaN
        if value.is_nan() {
            return;
        }

        self.inner.observe(value);
    }

    /// Provides temporary access to a snapshot of the gauge histogram's current state.
    ///
    /// # Parameters
    ///
    /// * `func` - A closure that receives a reference to the [`HistogramSnapshot`].
    ///
    /// # Returns
    ///
    /// The value returned by the provided closure
    ///
    /// # Example
    ///
    /// ```
    /// # use fastmetrics::metrics::gauge_histogram::GaugeHistogram;
    /// #
    /// let histogram = GaugeHistogram::default();
    /// histogram.observe(42.0);
    ///
    /// histogram.with_snapshot(|s| {
    ///     assert_eq!(s.count(), 1);
    ///     assert_eq!(s.sum(), 42.0);
    /// });
    /// ```
    pub fn with_snapshot<F, R>(&self, func: F) -> R
    where
        F: FnOnce(&HistogramSnapshot) -> R,
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
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> Result<()> {
        self.with_snapshot(|s| {
            let buckets = s.buckets();
            let exemplars = None;
            encoder.encode_gauge_histogram(buckets, exemplars, s.count(), s.sum())
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
            assert_eq!(s.count(), 0);
            assert_eq!(s.sum(), 0.0);
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
            assert_eq!(s.count(), 4);
            assert_eq!(s.sum(), 850.0);
        });
    }

    #[test]
    fn test_gauge_histogram_invalid_observations() {
        let hist = GaugeHistogram::default();

        hist.observe(-1.0); // Negative value, valid
        hist.observe(f64::NAN); // NaN value, invalid

        hist.with_snapshot(|s| {
            assert_eq!(s.count(), 1);
            assert_eq!(s.sum(), -1.0);
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
            assert_eq!(s.count(), 400);
            assert_eq!(s.sum(), 0.0);
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
