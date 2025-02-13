//! [Open Metrics Counter](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#counter) metric type.

use std::{
    cell::Cell,
    fmt::{self, Debug},
    ops::AddAssign,
    sync::{atomic::*, Arc},
    time::{Duration, SystemTime},
};

pub use crate::metrics::raw::{Atomic, Number};
use crate::metrics::{MetricType, TypedMetric};

/// A marker trait for **counter** metric value.
pub trait CounterValue<Rhs = Self>: Number + AddAssign<Rhs> {
    /// The atomic type corresponding to this counter value.
    type Atomic: Atomic<Self>;
}

macro_rules! impl_counter_value_for {
    ($($num:ident => $atomic:ident),*) => ($(
        impl CounterValue for $num {
            type Atomic = $atomic;
        }
    )*);
}

impl_counter_value_for! {
    u32 => AtomicU32,
    u64 => AtomicU64,
    usize => AtomicUsize,
    f32 => AtomicU32,
    f64 => AtomicU64
}

/// Open Metrics [`Counter`] metric, which is used to measure discrete events.
pub struct Counter<N: CounterValue = u64> {
    total: Arc<N::Atomic>,
    // UNIX timestamp
    created: Option<Duration>,
}

impl<N: CounterValue> Clone for Counter<N> {
    fn clone(&self) -> Self {
        Self { total: self.total.clone(), created: self.created }
    }
}

impl<N: CounterValue> Debug for Counter<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (total, created) = self.get();

        f.debug_struct("Counter")
            .field("total", &total)
            .field("created", &created)
            .finish()
    }
}

impl<N: CounterValue> Default for Counter<N> {
    fn default() -> Self {
        Self { total: Arc::new(Default::default()), created: None }
    }
}

impl<N: CounterValue> Counter<N> {
    /// Creates a [`Counter`] with a `created` timestamp.
    pub fn with_created() -> Self {
        Self {
            total: Default::default(),
            created: Some(
                SystemTime::UNIX_EPOCH
                    .elapsed()
                    .expect("UNIX timestamp when the counter was created"),
            ),
        }
    }

    /// Increases the [`Counter`] by 1, returning the previous value.
    #[inline]
    pub fn inc(&self) -> N {
        self.total.inc()
    }

    /// Increases the [`Counter`] by `v`, returning the previous value.
    #[inline]
    pub fn inc_by(&self, v: N) -> N {
        assert!(v >= N::ZERO);
        self.total.inc_by(v)
    }

    /// Gets the current `total` value and optional `created` value of the [`Counter`].
    #[inline]
    pub fn get(&self) -> (N, Option<Duration>) {
        (self.total.get(), self.created)
    }
}

impl<N: CounterValue> TypedMetric for Counter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

/// A **constant** [`Counter`], meaning it cannot be changed once created.
#[derive(Clone)]
pub struct ConstCounter<N = u64> {
    total: N,
    // UNIX timestamp
    created: Option<Duration>,
}

impl<N: CounterValue> Debug for ConstCounter<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (total, created) = self.get();

        f.debug_struct("ConstCounter")
            .field("total", &total)
            .field("created", &created)
            .finish()
    }
}

impl<N: CounterValue> ConstCounter<N> {
    /// Creates a new [`ConstCounter`] with a constant `total` value.
    pub const fn new(total: N) -> Self {
        Self { total, created: None }
    }

    /// Creates a [`ConstCounter`] with a constant `total` value and a `created` timestamp.
    pub fn with_created(total: N) -> Self {
        Self {
            total,
            created: Some(
                SystemTime::UNIX_EPOCH
                    .elapsed()
                    .expect("UNIX timestamp when the counter was created"),
            ),
        }
    }

    /// Gets the current `total` value and optional `created` value of the [`ConstCounter`].
    #[inline]
    pub const fn get(&self) -> (N, Option<Duration>) {
        (self.total, self.created)
    }
}

impl<N: CounterValue> TypedMetric for ConstCounter<N> {
    const TYPE: MetricType = MetricType::Counter;
}

/// An **unsync** [`Counter`], meaning it can only be used in single-thread environment.
pub struct LocalCounter<N = u64> {
    total: Cell<N>,
    // UNIX timestamp
    created: Option<Duration>,
}

impl<N: CounterValue> Clone for LocalCounter<N> {
    fn clone(&self) -> Self {
        Self { total: self.total.clone(), created: self.created }
    }
}

impl<N: CounterValue> Debug for LocalCounter<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (total, created) = self.get();

        f.debug_struct("LocalCounter")
            .field("total", &total)
            .field("created", &created)
            .finish()
    }
}

impl<N: CounterValue> Default for LocalCounter<N> {
    fn default() -> Self {
        Self { total: Cell::new(N::ZERO), created: None }
    }
}

impl<N: CounterValue> LocalCounter<N> {
    /// Creates a [`LocalCounter`] with a `created` timestamp.
    pub fn with_created() -> Self {
        Self {
            total: Cell::new(N::ZERO),
            created: Some(
                SystemTime::UNIX_EPOCH
                    .elapsed()
                    .expect("UNIX timestamp when the counter was created"),
            ),
        }
    }

    /// Increases the [`LocalCounter`] by 1, returning the previous value.
    #[inline]
    pub fn inc(&self) -> N {
        self.inc_by(N::ONE)
    }

    /// Increases the [`LocalCounter`] by `v`, returning the previous value.
    #[inline]
    pub fn inc_by(&self, v: N) -> N {
        assert!(v >= N::ZERO);
        let (mut new, _) = self.get();
        new += v;
        self.total.replace(new)
    }

    /// Gets the current `total` value and optional `created` value of the [`LocalCounter`].
    #[inline]
    pub fn get(&self) -> (N, Option<Duration>) {
        (self.total.get(), self.created)
    }
}

impl<N: CounterValue> TypedMetric for LocalCounter<N> {
    const TYPE: MetricType = MetricType::Counter;
}
