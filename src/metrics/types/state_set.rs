//! [Open Metrics StateSet](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#stateset) metric type.

use std::{
    cell::Cell,
    fmt::{self, Debug},
    marker::PhantomData,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};

use crate::metrics::{MetricType, TypedMetric};

/// A marker trait for **stateset** metric value.
pub trait StateSetValue: Sized + PartialEq + 'static {
    /// Return all variants of [`StateSet`] value.
    fn variants() -> &'static [Self];
    /// Return the string representation for the [`StateSet`] value.
    fn as_str(&self) -> &str;
}

/// Open Metrics [`StateSet`] metric, which represent a series of related boolean values, also
/// called a bitset.
#[derive(Clone)]
pub struct StateSet<T> {
    current_state: Arc<AtomicU8>,
    _marker: PhantomData<T>,
}

impl<T: StateSetValue + Debug> Debug for StateSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.current();
        f.debug_struct("StateSet").field("state", state).finish()
    }
}

impl<T: StateSetValue + Default> Default for StateSet<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: StateSetValue> StateSet<T> {
    /// Creates a [`StateSet`] with the given initial state.
    pub fn new(initial_state: T) -> Self {
        let pos = find_position(initial_state);
        Self { current_state: Arc::new(AtomicU8::new(pos)), _marker: PhantomData }
    }

    /// Sets the current state.
    pub fn set(&self, state: T) {
        let pos = find_position(state);
        self.current_state.store(pos, Ordering::Relaxed);
    }

    /// Returns the current state.
    pub fn current(&self) -> &T {
        let index = self.current_state.load(Ordering::Relaxed) as usize;
        T::variants().get(index).expect("Invalid state index")
    }

    /// Returns the all states.
    pub fn get(&self) -> Vec<(&str, bool)> {
        let current = self.current();
        gen_states(current)
    }
}

impl<T: StateSetValue> TypedMetric for StateSet<T> {
    const TYPE: MetricType = MetricType::StateSet;
}

/// A **constant** [`StateSet`], meaning it cannot be changed once created.
#[derive(Clone, Debug)]
pub struct ConstStateSet<T> {
    state: T,
}

impl<T: StateSetValue + Default> Default for ConstStateSet<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: StateSetValue> ConstStateSet<T> {
    /// Creates a [`ConstStateSet`] with the given initial state.
    pub fn new(initial_state: T) -> Self {
        Self { state: initial_state }
    }

    /// Returns the current state.
    pub fn current(&self) -> &T {
        &self.state
    }

    /// Returns the all states.
    pub fn get(&self) -> Vec<(&str, bool)> {
        let current = self.current();
        gen_states(current)
    }
}

impl<T: StateSetValue> TypedMetric for ConstStateSet<T> {
    const TYPE: MetricType = MetricType::StateSet;
}

/// An **unsync** [`StateSet`], meaning it can only be used in single-thread environment.
#[derive(Clone)]
pub struct LocalStateSet<T> {
    current_state: Cell<u8>,
    _marker: PhantomData<T>,
}

impl<T: StateSetValue + Debug> Debug for LocalStateSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.current();
        f.debug_struct("LocalStateSet").field("state", state).finish()
    }
}

impl<T: StateSetValue + Default> Default for LocalStateSet<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: StateSetValue> LocalStateSet<T> {
    /// Creates a [`LocalStateSet`] with the given initial state.
    pub fn new(initial_state: T) -> Self {
        let pos = find_position(initial_state);
        Self { current_state: Cell::new(pos), _marker: PhantomData }
    }

    /// Sets the current state.
    pub fn set(&self, state: T) {
        let pos = find_position(state);
        self.current_state.set(pos);
    }

    /// Returns the current state.
    pub fn current(&self) -> &T {
        let index = self.current_state.get() as usize;
        T::variants().get(index).expect("Invalid state index")
    }

    /// Returns the all states.
    pub fn get(&self) -> Vec<(&str, bool)> {
        let current = self.current();
        gen_states(current)
    }
}

impl<T: StateSetValue> TypedMetric for LocalStateSet<T> {
    const TYPE: MetricType = MetricType::StateSet;
}

fn find_position<T: StateSetValue>(state: T) -> u8 {
    T::variants()
        .iter()
        .position(|s| s == &state)
        .expect("State must exist in variants") as u8
}

fn gen_states<T: StateSetValue>(current: &T) -> Vec<(&str, bool)> {
    T::variants()
        .iter()
        .map(|variant| {
            let is_active = variant == current;
            (variant.as_str(), is_active)
        })
        .collect::<Vec<_>>()
}
