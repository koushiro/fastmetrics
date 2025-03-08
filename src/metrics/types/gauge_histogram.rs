//! [Open Metrics GaugeHistogram](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#gaugehistogram) metric type.
//!
//! See [`GaugeHistogram`] for more details.

use std::{
    fmt::{self, Debug},
    sync::Arc,
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
/// use openmetrics_client::metrics::gauge_histogram::{linear_buckets, GaugeHistogram};
/// // Create a histogram with custom bucket boundaries
/// let hist = GaugeHistogram::new([-273.15, -200.0, -100.0, 0.0, 100.0, 200.0]);
///
/// // Observe some values
/// hist.observe(-250.0);   // Falls into ≤-200 bucket
/// hist.observe(0.0);      // Falls into ≤0.0 bucket
/// hist.observe(100.0);    // Falls into ≤100.0 bucket
/// hist.observe(1000.0);   // Falls into +Inf bucket
///
/// // Get bucket counts
/// let buckets = hist.buckets();
/// assert_eq!(buckets[1].upper_bound(), -200.0);
/// assert_eq!(buckets[1].count(), 1);  // One value ≤-200.0
/// assert_eq!(buckets[3].upper_bound(), 0.0);
/// assert_eq!(buckets[3].count(), 1);  // One value ≤0.0
/// assert_eq!(buckets[4].upper_bound(), 100.0);
/// assert_eq!(buckets[4].count(), 1);  // One value ≤100.0
/// assert_eq!(buckets[6].upper_bound(), f64::INFINITY);
/// assert_eq!(buckets[6].count(), 1);  // One value in +Inf bucket
///
/// // Get count and sum statistics
/// assert_eq!(hist.gcount(), 4);       // Total number of observations
/// assert_eq!(hist.gsum(), 850.0);     // Sum of all observed values
/// ```
#[derive(Clone)]
pub struct GaugeHistogram {
    inner: Arc<RwLock<GaugeHistogramInner>>,
}

struct GaugeHistogramInner {
    buckets: Vec<Bucket>,
    gsum: f64,
    gcount: u64,
}

impl Debug for GaugeHistogram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.read();
        let buckets = &inner.buckets;
        let gsum = inner.gsum;
        let gcount = inner.gcount;

        f.debug_struct("GaugeHistogram")
            .field("buckets", buckets)
            .field("gsum", &gsum)
            .field("gcount", &gcount)
            .finish()
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
            inner: Arc::new(RwLock::new(GaugeHistogramInner { buckets, gsum: 0f64, gcount: 0 })),
        }
    }

    /// Observes a value, incrementing the appropriate buckets.
    pub fn observe(&self, value: f64) {
        // Sum and bucket values MUST NOT be NaN or negative
        if value.is_nan() {
            return;
        }

        let mut inner = self.inner.write();
        // Increment the gcount and add the value into the gsum
        inner.gcount += 1;
        inner.gsum += value;

        // Only increment the count of the found bucket
        let idx = inner.buckets.partition_point(|bucket| bucket.upper_bound() < value);
        inner.buckets[idx].inc();
    }

    /// Gets the current `bucket` counts.
    pub fn buckets(&self) -> Vec<Bucket> {
        self.inner.read().buckets.clone()
    }

    /// Gets the current `gsum` of all observed values.
    pub fn gsum(&self) -> f64 {
        self.inner.read().gsum
    }

    /// Gets the current `gcount` of all observations.
    pub fn gcount(&self) -> u64 {
        self.inner.read().gcount
    }
}

impl TypedMetric for GaugeHistogram {
    const TYPE: MetricType = MetricType::GaugeHistogram;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauge_histogram_initialization() {
        let hist = GaugeHistogram::default();
        let buckets = hist.buckets();
        assert_eq!(buckets.len(), DEFAULT_BUCKETS.len() + 1); // Including +Inf bucket
        assert_eq!(hist.gsum(), 0.0);
        assert_eq!(hist.gcount(), 0);

        let bounds = vec![1.0, 2.0, 5.0];
        let hist = GaugeHistogram::new(bounds);
        let buckets = hist.buckets();
        assert_eq!(buckets.len(), 4); // Including +Inf bucket
        assert_eq!(buckets[0].upper_bound(), 1.0);
        assert_eq!(buckets[1].upper_bound(), 2.0);
        assert_eq!(buckets[2].upper_bound(), 5.0);
        assert_eq!(buckets[3].upper_bound(), f64::INFINITY);
    }

    #[test]
    fn test_gauge_histogram_observe() {
        let hist = GaugeHistogram::new(vec![-273.15, -200.0, -100.0, 0.0, 100.0, 200.0]);

        hist.observe(-250.0);
        hist.observe(0.0);
        hist.observe(100.0);
        hist.observe(1000.0);

        let buckets = hist.buckets();
        assert_eq!(buckets[1].count(), 1); // ≤-200.0
        assert_eq!(buckets[3].count(), 1); // ≤0.0
        assert_eq!(buckets[4].count(), 1); // ≤100.0
        assert_eq!(buckets[6].count(), 1); // +Inf
        assert_eq!(hist.gcount(), 4);
        assert_eq!(hist.gsum(), 850.0);
    }

    #[test]
    fn test_gauge_histogram_invalid_observations() {
        let hist = GaugeHistogram::default();

        hist.observe(-1.0); // Negative value, valid
        hist.observe(f64::NAN); // NaN value, invalid

        assert_eq!(hist.gcount(), 1);
        assert_eq!(hist.gsum(), -1.0);
    }

    #[test]
    fn test_gauge_histogram_thread_safe() {
        let hist = GaugeHistogram::new(vec![-273.15, -200.0, -100.0, 0.0, 100.0, 200.0]);
        let clone = hist.clone();

        let handle = std::thread::spawn(move || {
            for i in -100..1000 {
                clone.observe(i as f64);
            }
        });

        for i in -100..1000 {
            hist.observe(i as f64);
        }

        handle.join().unwrap();
        assert_eq!(hist.gcount(), 2200);
    }
}
