//! [Open Metrics StateSet](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#stateset) metric type.
//!
//! See [`StateSet`] and [`ConstStateSet`] for more details.

use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    sync::{atomic::*, Arc},
};

#[cfg(feature = "derive")]
pub use fastmetrics_derive::StateSetValue;

use crate::raw::{MetricType, TypedMetric};

/// A marker trait for **stateset** metric value.
pub trait StateSetValue: Sized + PartialEq + 'static {
    /// Return all variants of [`StateSet`] value.
    fn variants() -> &'static [Self];
    /// Return the string representation for the [`StateSet`] value.
    fn as_str(&self) -> &str;
}

/// Open Metrics [`StateSet`] metric, which represent a series of related boolean values, also
/// called a bitset.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::state_set::{StateSet, StateSetValue};
/// #[derive(Copy, Clone, Debug, PartialEq, Default)]
/// enum JobState {
///     #[default]
///     Pending,
///     Running,
///     Completed,
///     Failed,
/// }
///
/// // Can use `#[derive(StateSetValue)]` to simplify the code, but need to enable `derive` feature
/// impl StateSetValue for JobState {
///     fn variants() -> &'static [Self] {
///         &[Self::Pending, Self::Running, Self::Completed, Self::Failed]
///     }
///
///     fn as_str(&self) -> &str {
///         match self {
///             Self::Pending => "Pending",
///             Self::Running => "Running",
///             Self::Completed => "Completed",
///             Self::Failed => "Failed",
///         }
///     }
/// }
///
/// // Create a default stateset (Pending)
/// let state = StateSet::<JobState>::default();
/// assert_eq!(state.get(), &JobState::Pending);
///
/// // Create a stateset with initial state
/// let state = StateSet::new(JobState::Running);
/// assert_eq!(state.get(), &JobState::Running);
///
/// // Change state
/// state.set(JobState::Completed);
/// assert_eq!(state.get(), &JobState::Completed);
///
/// // Get all states with their status
/// let states = state.states();
/// assert_eq!(states, vec![
///     ("Pending", false),
///     ("Running", false),
///     ("Completed", true),
///     ("Failed", false),
/// ]);
/// ```
#[derive(Clone)]
pub struct StateSet<T> {
    current_state: Arc<AtomicU8>,
    _marker: PhantomData<T>,
}

impl<T: StateSetValue + Debug> Debug for StateSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.get();
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

    /// Gets the current state.
    pub fn get(&self) -> &T {
        let index = self.current_state.load(Ordering::Relaxed) as usize;
        T::variants().get(index).expect("Invalid state index")
    }

    /// Returns the all states with exactly one Boolean value being true.
    pub fn states(&self) -> Vec<(&str, bool)> {
        let current = self.get();
        gen_states(current)
    }
}

impl<T: StateSetValue> TypedMetric for StateSet<T> {
    const TYPE: MetricType = MetricType::StateSet;
    const WITH_TIMESTAMP: bool = false;
}

/// A **constant** [`StateSet`], meaning it cannot be changed once created.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::metrics::state_set::{ConstStateSet, StateSetValue};
/// #[derive(Copy, Clone, Debug, PartialEq)]
/// enum JobState {
///     Pending,
///     Running,
///     Completed,
///     Failed,
/// }
///
/// // Can use `#[derive(StateSetValue)]` to simplify the code, but need to enable `derive` feature
/// impl StateSetValue for JobState {
///     fn variants() -> &'static [Self] {
///         &[Self::Pending, Self::Running, Self::Completed, Self::Failed]
///     }
///
///     fn as_str(&self) -> &str {
///         match self {
///             Self::Pending => "Pending",
///             Self::Running => "Running",
///             Self::Completed => "Completed",
///             Self::Failed => "Failed",
///         }
///     }
/// }
///
/// // Create a constant stateset with initial state
/// let state = ConstStateSet::new(JobState::Completed);
/// assert_eq!(state.get(), &JobState::Completed);
///
/// // Get all states with their status
/// let states = state.states();
/// assert_eq!(states, vec![
///     ("Pending", false),
///     ("Running", false),
///     ("Completed", true),
///     ("Failed", false),
/// ]);
/// ```
#[derive(Clone, Debug)]
pub struct ConstStateSet<T> {
    state: T,
}

impl<T: StateSetValue> ConstStateSet<T> {
    /// Creates a [`ConstStateSet`] with the given initial state.
    pub fn new(initial_state: T) -> Self {
        Self { state: initial_state }
    }

    /// Gets the current state.
    pub fn get(&self) -> &T {
        &self.state
    }

    /// Returns the all states.
    pub fn states(&self) -> Vec<(&str, bool)> {
        let current = self.get();
        gen_states(current)
    }
}

impl<T: StateSetValue> TypedMetric for ConstStateSet<T> {
    const TYPE: MetricType = MetricType::StateSet;
    const WITH_TIMESTAMP: bool = false;
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
            let enabled = variant == current;
            (variant.as_str(), enabled)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Default)]
    enum TestState {
        #[default]
        Pending,
        Running,
        Completed,
        Failed,
    }

    impl StateSetValue for TestState {
        fn variants() -> &'static [Self] {
            &[Self::Pending, Self::Running, Self::Completed, Self::Failed]
        }

        fn as_str(&self) -> &str {
            match self {
                Self::Pending => "pending",
                Self::Running => "running",
                Self::Completed => "completed",
                Self::Failed => "failed",
            }
        }
    }

    #[test]
    fn test_stateset_initialization() {
        let state = StateSet::<TestState>::default();
        assert_eq!(state.get(), &TestState::Pending);

        let state = StateSet::new(TestState::Running);
        assert_eq!(state.get(), &TestState::Running);
    }

    #[test]
    fn test_stateset_set() {
        let state = StateSet::default();
        let clone = state.clone();

        state.set(TestState::Running);
        assert_eq!(state.get(), &TestState::Running);
        assert_eq!(clone.get(), &TestState::Running);

        clone.set(TestState::Completed);
        assert_eq!(state.get(), &TestState::Completed);
    }

    #[test]
    fn test_stateset_states() {
        let state = StateSet::new(TestState::Running);
        let states = state.states();
        assert_eq!(
            states,
            vec![("pending", false), ("running", true), ("completed", false), ("failed", false)]
        );
    }

    #[test]
    fn test_state_set_thread_safe() {
        let state = StateSet::new(TestState::Pending);
        let clone = state.clone();

        let handle = std::thread::spawn(move || {
            clone.set(TestState::Running);
        });

        handle.join().unwrap();
        assert_eq!(state.get(), &TestState::Running);
    }

    #[test]
    fn test_const_stateset() {
        let state = ConstStateSet::new(TestState::Running);
        let clone = state.clone();
        assert_eq!(state.get(), &TestState::Running);
        assert_eq!(clone.get(), &TestState::Running);

        let states = state.states();
        assert_eq!(
            states,
            vec![("pending", false), ("running", true), ("completed", false), ("failed", false)]
        );
    }
}
