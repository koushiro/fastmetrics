#[cfg(feature = "derive")]
pub use fastmetrics_derive::Register;

use super::{Registry, RegistryError, with_global_registry_mut};

/// A trait for types that can be registered into a [`Registry`].
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::counter::Counter,
/// #     registry::{Register, Registry, RegistryError},
/// # };
/// #[derive(Clone, Default)]
/// struct Metrics {
///     counter: Counter,
/// }
///
/// impl Register for Metrics {
///     fn register(&self, registry: &mut Registry) -> Result<(), RegistryError> {
///         registry.register("my_counter", "My counter help", self.counter.clone())?;
///         Ok(())
///     }
/// }
///
/// fn main() -> Result<(), RegistryError> {
///     let mut registry = Registry::default();
///     let metrics = Metrics::default();
///     metrics.register(&mut registry)?;
///     // ...
///     Ok(())
/// }
/// ```
pub trait Register {
    /// Registers the implementing type into the provided [`Registry`].
    fn register(&self, registry: &mut Registry) -> Result<(), RegistryError>;

    /// Registers the implementing type into the global [`Registry`].
    fn register_global(&self) -> Result<(), RegistryError> {
        with_global_registry_mut(|registry| self.register(registry))
    }
}
