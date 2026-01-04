//! Shared internal implementation for histogram-like metric types.
//!
//! It exists to reduce duplication between `Histogram` and `GaugeHistogram`
//! while keeping their externally-visible semantics intact.

use std::sync::atomic::AtomicU64;

use crate::raw::Atomic;
pub use crate::raw::bucket::Bucket;

/// Controls which bucket bounds are accepted.
#[derive(Clone, Copy, Debug)]
pub enum BoundsFilter {
    /// Reject NaN bounds; keep both negative and positive bounds.
    AllowNegative,
    /// Reject NaN bounds and also reject negative bounds.
    RejectNegative,
}

/// Histogram-like core holding bucket counters plus `(count,sum)` accumulators.
///
/// Notes:
/// - Buckets always include a `+Inf` upper bound.
/// - Bucket counts are **non-cumulative** (each observation increments exactly one bucket).
/// - `sum` is stored as an `AtomicU64` containing the IEEE754 bits of an accumulated `f64`
///   using the crate's `raw::Atomic` extension methods.
pub struct HistogramCore {
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
        self.count.inc_by(1);
    }

    fn load(&self) -> Bucket {
        Bucket::new(self.upper_bound, self.count.get())
    }
}

impl HistogramCore {
    pub fn from_bounds(buckets: impl IntoIterator<Item = f64>, filter: BoundsFilter) -> Self {
        let mut upper_bounds = buckets
            .into_iter()
            .filter(|upper_bound| {
                if upper_bound.is_nan() {
                    return false;
                }
                match filter {
                    BoundsFilter::AllowNegative => true,
                    BoundsFilter::RejectNegative => upper_bound.is_sign_positive(),
                }
            })
            .collect::<Vec<_>>();

        // sort and dedup the bounds
        upper_bounds.sort_by(|a, b| a.partial_cmp(b).expect("upper_bound must not be NaN"));
        upper_bounds.dedup();

        // ensure +Inf bucket is included
        match upper_bounds.last() {
            Some(last) if last.is_finite() => upper_bounds.push(f64::INFINITY),
            None => upper_bounds.push(f64::INFINITY),
            _ => { /* already +Inf */ },
        }

        let buckets = upper_bounds.into_iter().map(BucketCell::new).collect::<Vec<_>>();

        Self { buckets, count: AtomicU64::new(0), sum: AtomicU64::new(0f64.to_bits()) }
    }

    pub fn observe(&self, value: f64) {
        // Increment count and sum
        self.count.inc_by(1);
        self.sum.inc_by(value);

        // Increment only the found bucket
        let idx = self.bucket_index(value);
        self.buckets[idx].inc();
    }

    pub fn bucket_index(&self, value: f64) -> usize {
        self.buckets.partition_point(|bucket| bucket.upper_bound < value)
    }

    pub fn snapshot(&self) -> HistogramSnapshot {
        let buckets = self.buckets.iter().map(BucketCell::load).collect();
        let count = self.count.get();
        let sum = self.sum.get();
        HistogramSnapshot { buckets, count, sum }
    }
}

/// A snapshot of a histogram-like metric at a point in time.
///
/// Notes:
/// - `buckets()` returns the current per-bucket counts (non-cumulative).
/// - `count()` is the total number of observations.
/// - `sum()` is the sum of all observed values.
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

    /// Gets the current total `count` of all observations.
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Gets the current `sum` of all observed values.
    pub const fn sum(&self) -> f64 {
        self.sum
    }
}
