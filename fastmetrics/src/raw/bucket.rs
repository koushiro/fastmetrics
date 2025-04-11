//! Provides bucket-related functionality for histogram metrics in OpenMetrics.
//!
//! This module contains implementations for histogram buckets, which are used to track
//! the distribution of observations in histogram metrics. It also provides utilities
//! for generating different types of bucket distributions (linear and exponential).

use std::iter;

/// The label that defines the upper bound of a bucket of a histogram ("le" -> "less or equal").
pub const BUCKET_LABEL: &str = "le";

/// The default buckets are tailored to broadly measure the response time (in seconds) of a network
/// service.
/// Most likely, however, you will be required to define buckets customized to your use case.
pub const DEFAULT_BUCKETS: [f64; 11] =
    [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];

/// A histogram bucket that tracks the count of observations within its bounds.
///
/// Each bucket is defined by an upper bound and maintains a count of observations
/// that fall into the range (previous_bucket_upper_bound, current_bucket_upper_bound].
/// For the first bucket, the range starts at negative infinity.
#[derive(Copy, Clone, Debug)]
pub struct Bucket {
    upper_bound: f64,
    count: u64,
}

impl Bucket {
    /// Creates a new histogram bucket with the specified upper bound and initial count.
    ///
    /// # Arguments
    ///
    /// * `upper_bound` - The upper bound of this bucket
    /// * `count` - The initial count of observations in this bucket
    pub const fn new(upper_bound: f64, count: u64) -> Self {
        Self { upper_bound, count }
    }

    /// Increments the observation count in this bucket by one.
    ///
    /// This method is typically called when a new observation falls within this bucket's range.
    pub fn inc(&mut self) {
        self.count += 1;
    }

    /// Returns the upper bound of this bucket.
    ///
    /// The upper bound defines the maximum value that can be counted in this bucket.
    /// Values less than or equal to this bound (but greater than the previous bucket's
    /// upper bound) will be counted in this bucket.
    pub const fn upper_bound(&self) -> f64 {
        self.upper_bound
    }

    /// Returns the current count of observations in this bucket.
    ///
    /// This represents the number of observations that fall within this bucket's range.
    pub const fn count(&self) -> u64 {
        self.count
    }
}

/// Creates linearly spaced histogram buckets.
///
/// This function generates `count` number of buckets, where each bucket's upper bound
/// increases by `width` from the `start` value.
///
/// # Arguments
///
/// * `start` - The upper bound of the first bucket
/// * `width` - The width (linear step) between each bucket
/// * `count` - The number of buckets to generate (must be at least 1)
///
/// # Panics
///
/// Panics if:
/// * `width` is not positive
/// * `count` is less than 1
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::histogram::linear_buckets;
///
/// let buckets: Vec<f64> = linear_buckets(0.0, 1.0, 5).collect();
/// assert_eq!(buckets, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
/// ```
pub fn linear_buckets(start: f64, width: f64, count: usize) -> impl Iterator<Item = f64> {
    assert!(width > 0.0, "linear buckets must have a width greater than 0");
    assert!(count >= 1, "linear buckets must have at least 1 bucket");

    iter::repeat(())
        .enumerate()
        .map(move |(step, _)| start + (width * (step as f64)))
        .take(count)
}

/// Creates exponentially spaced histogram buckets.
///
/// This function generates `count` number of buckets, where each bucket's upper bound
/// increases by multiplying the previous bound by `factor`.
///
/// # Arguments
///
/// * `start` - The upper bound of the first bucket (must be positive)
/// * `factor` - The factor to multiply each bucket by (must be greater than 1)
/// * `count` - The number of buckets to generate (must be at least 1)
///
/// # Panics
///
/// Panics if:
/// * `start` is not positive
/// * `factor` is not greater than 1
/// * `count` is less than 1
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::histogram::exponential_buckets;
///
/// let buckets: Vec<f64> = exponential_buckets(1.0, 2.0, 4).collect();
/// assert_eq!(buckets, vec![1.0, 2.0, 4.0, 8.0]);
/// ```
pub fn exponential_buckets(start: f64, factor: f64, count: usize) -> impl Iterator<Item = f64> {
    assert!(start > 0.0, "exponential buckets must have a positive start value");
    assert!(factor > 1.0, "exponential buckets must have a factor greater than 1");
    assert!(count >= 1, "exponential buckets must have at least 1 bucket");

    iter::repeat(())
        .enumerate()
        .map(move |(next, _)| start * factor.powi(next as i32))
        .take(count)
}

/// Creates exponentially spaced histogram buckets between a minimum and maximum value.
///
/// This function generates `count` number of buckets, where the first bucket starts at `min`,
/// the last bucket ends at `max`, and intermediate buckets are spaced exponentially.
///
/// # Arguments
///
/// * `min` - The lower bound of the first bucket (must be positive)
/// * `max` - The upper bound of the last bucket
/// * `count` - The number of buckets to generate (must be at least 1)
///
/// # Panics
///
/// Panics if:
/// * `min` is not positive
/// * `count` is less than 1
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::histogram::exponential_buckets_range;
///
/// let buckets: Vec<f64> = exponential_buckets_range(1.0, 8.0, 4).collect();
/// assert_eq!(buckets, vec![1.0, 2.0, 4.0, 8.0]);
/// ```
pub fn exponential_buckets_range(min: f64, max: f64, count: usize) -> impl Iterator<Item = f64> {
    assert!(min > 0.0, "exponential buckets must have a positive min value");
    assert!(count >= 1, "exponential buckets must have at least 1 bucket");

    // max = min * growth_factor^(bucket_count-1)
    // => growth_factor = (max/min)^(1/(bucket_count - 1))
    let factor = (max / min).powf(1.0 / (count - 1) as f64);

    iter::repeat(())
        .enumerate()
        .map(move |(next, _)| min * factor.powi(next as i32))
        .take(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_linear_buckets() {
        let cases = [(
            // linear bucket params (start, width, count)
            (0.0, 1.0, 10),
            // expected linear buckets
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        )];

        for ((start, width, count), expected) in cases {
            let got = linear_buckets(start, width, count).collect::<Vec<_>>();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn gen_exponential_buckets() {
        let cases = [
            (
                // exponential bucket params (start, factor, count)
                (1.0, 2.0, 10),
                // expected exponential buckets
                vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0],
            ),
            (
                (1.0, 3.0, 10),
                vec![1.0, 3.0, 9.0, 27.0, 81.0, 243.0, 729.0, 2187.0, 6561.0, 19683.0],
            ),
            ((1.0, 2.0, 1), vec![1.0]),
        ];

        for ((start, factor, count), expected) in cases {
            let got = exponential_buckets(start, factor, count).collect::<Vec<_>>();
            assert_eq!(got, expected);
        }
    }

    #[test]
    fn gen_exponential_buckets_range() {
        let cases = [
            (
                // exponential bucket params (min, max, count)
                (1.0, 512.0, 10),
                // expected exponential buckets
                vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0],
            ),
            (
                (1.0, 19683.0, 10),
                vec![1.0, 3.0, 9.0, 27.0, 81.0, 243.0, 729.0, 2187.0, 6561.0, 19683.0],
            ),
            ((1.0, 4.0, 1), vec![1.0]),
        ];

        for ((start, factor, count), expected) in cases {
            let got = exponential_buckets_range(start, factor, count).collect::<Vec<_>>();
            assert_eq!(got, expected);
        }
    }
}
