use crate::{
    error::Result,
    registry::{Registry, with_global_registry_mut},
};

/// A trait for types that can be registered into a [`Registry`].
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #     error::Result,
/// #     metrics::counter::Counter,
/// #     registry::{Register, Registry},
/// # };
/// #
/// #[derive(Clone, Default)]
/// struct Metrics {
///     counter: Counter,
/// }
///
/// impl Register for Metrics {
///     fn register(&self, registry: &mut Registry) -> Result<()> {
///         registry.register("my_counter", "My counter help", self.counter.clone())?;
///         Ok(())
///     }
/// }
///
/// fn main() -> Result<()> {
///     let mut registry = Registry::default();
///     let metrics = Metrics::default();
///     metrics.register(&mut registry)?;
///     // ...
///     Ok(())
/// }
/// ```
pub trait Register {
    /// Registers the implementing type into the provided [`Registry`].
    fn register(&self, registry: &mut Registry) -> Result<()>;

    /// Registers the implementing type into the global [`Registry`].
    fn register_global(&self) -> Result<()> {
        with_global_registry_mut(|registry| self.register(registry))
    }
}
