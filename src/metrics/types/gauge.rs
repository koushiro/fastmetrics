//! [Open Metrics Gauge](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#gauge) metric type.

use std::{
    fmt::{self, Debug},
    ops::{AddAssign, SubAssign},
    sync::{atomic::*, Arc},
};

use crate::metrics::{
    raw::{Atomic, Number},
    MetricType, TypedMetric,
};

/// A marker trait for **gauge** metric value.
pub trait GaugeValue<Rhs = Self>: Number + AddAssign<Rhs> + SubAssign<Rhs> {
    /// The atomic type corresponding to this gauge value.
    type Atomic: Atomic<Self>;
}

macro_rules! impl_gauge_value_for {
    ($($num:ident => $atomic:ident),*) => ($(
        impl GaugeValue for $num {
            type Atomic = $atomic;
        }
    )*);
}

impl_gauge_value_for! {
    i32 => AtomicI32,
    i64 => AtomicI64,
    isize => AtomicIsize,
    u32 => AtomicU32,
    f32 => AtomicU32,
    f64 => AtomicU64
}

/// Open Metrics [`Gauge`] metric, which is used to record current measurements,
/// such as bytes of memory currently used or the number of items in a queue.
pub struct Gauge<N: GaugeValue = i64> {
    value: Arc<N::Atomic>,
}

impl<N: GaugeValue> Clone for Gauge<N> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<N: GaugeValue> Debug for Gauge<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gauge").field("value", &self.get()).finish()
    }
}

impl<N: GaugeValue> Default for Gauge<N> {
    fn default() -> Self {
        Self { value: Arc::new(Default::default()) }
    }
}

impl<N: GaugeValue> Gauge<N> {
    /// Creates a new [`Gauge`] with an initial value.
    pub fn new(value: N) -> Self {
        let this = Self::default();
        this.set(value);
        this
    }

    /// Increases the [`Gauge`] by 1, returning the previous value.
    #[inline]
    pub fn inc(&self) -> N {
        self.value.inc()
    }

    /// Increases the [`Gauge`] by `v`, returning the previous value.
    #[inline]
    pub fn inc_by(&self, v: N) -> N {
        assert!(v >= N::ZERO);
        self.value.inc_by(v)
    }

    /// Decreases the [`Gauge`] by 1, returning the previous value.
    #[inline]
    pub fn dec(&self) -> N {
        self.value.dec()
    }

    /// Decreases the [`Gauge`] by `v`, returning the previous value.
    #[inline]
    pub fn dec_by(&self, v: N) -> N {
        assert!(v >= N::ZERO);
        self.value.dec_by(v)
    }

    /// Sets the [`Gauge`] to `v`, returning the previous value.
    #[inline]
    pub fn set(&self, v: N) -> N {
        self.value.set(v)
    }

    /// Gets the current value of the [`Gauge`].
    #[inline]
    pub fn get(&self) -> N {
        self.value.get()
    }
}

impl<N: GaugeValue> TypedMetric for Gauge<N> {
    const TYPE: MetricType = MetricType::Gauge;
}

/// A **constant** [`Gauge`], meaning it cannot be changed once created.
#[derive(Clone, Debug)]
pub struct ConstGauge<N = i64> {
    value: N,
}

impl<N: GaugeValue> ConstGauge<N> {
    /// Creates a new [`ConstGauge`] with a constant value.
    pub const fn new(value: N) -> Self {
        Self { value }
    }

    /// Gets the current value of the [`ConstGauge`].
    #[inline]
    pub const fn get(&self) -> N {
        self.value
    }
}

impl<N: GaugeValue> TypedMetric for ConstGauge<N> {
    const TYPE: MetricType = MetricType::Gauge;
}
