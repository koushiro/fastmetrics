use std::{borrow::Cow, error, fmt, sync::OnceLock};

use parking_lot::RwLock;

use crate::registry::{Metric, Registry, RegistryError, Unit};

/// Error returned when trying to set a global registry when another has already been initialized.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SetRegistryError;

impl fmt::Display for SetRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Global registry has already been initialized")
    }
}

impl error::Error for SetRegistryError {}

struct GlobalRegistry {
    registry: OnceLock<RwLock<Registry>>,
}

impl GlobalRegistry {
    const fn new() -> Self {
        Self { registry: OnceLock::new() }
    }
}

trait RegistryProvider: Send + Sync {
    fn set(&self, registry: Registry) -> Result<(), SetRegistryError>;

    fn get(&self) -> &RwLock<Registry>;
}

impl RegistryProvider for GlobalRegistry {
    fn set(&self, registry: Registry) -> Result<(), SetRegistryError> {
        self.registry.set(RwLock::new(registry)).map_err(|_| SetRegistryError)
    }

    fn get(&self) -> &RwLock<Registry> {
        self.registry.get_or_init(|| RwLock::new(Registry::default()))
    }
}

static GLOBAL_REGISTRY: GlobalRegistry = GlobalRegistry::new();

#[cfg(test)]
thread_local! {
    static TEST_REGISTRY: std::cell::RefCell<Option<&'static dyn RegistryProvider>> = std::cell::RefCell::new(None);
}

fn registry_provider() -> &'static dyn RegistryProvider {
    #[cfg(not(test))]
    {
        &GLOBAL_REGISTRY
    }

    #[cfg(test)]
    {
        TEST_REGISTRY.with(|reg| reg.borrow().unwrap_or(&GLOBAL_REGISTRY))
    }
}

/// Sets the global registry to the provided registry instance.
///
/// This function allows you to replace the default global registry with a custom one.
/// It can only be called once - subsequent calls will return [`SetRegistryError`].
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from multiple threads simultaneously.
/// However, only the first successful call will set the registry.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::registry::{Registry, set_global_registry};
/// let custom_registry = Registry::builder()
///     .with_namespace("myapp")
///     .with_const_labels([("env", "prod")])
///     .build();
///
///
/// // This will succeed
/// assert!(set_global_registry(custom_registry).is_ok());
/// // metric operations
/// // ...
///
/// // This will fail
/// let another_registry = Registry::builder()
///     .with_namespace("other")
///     .build();
/// assert!(set_global_registry(another_registry).is_err());
/// ```
pub fn set_global_registry(registry: Registry) -> Result<(), SetRegistryError> {
    let provider = registry_provider();
    provider.set(registry)
}

/// Executes a function with read-only access to the global registry.
///
/// This function provides safe, read-only access to the global registry without
/// exposing the underlying synchronization primitives. The global registry will
/// be initialized with default settings if it hasn't been set previously.
///
/// # Arguments
///
/// * `f` - A closure that takes a reference to the [`Registry`] and returns a value of type `R`
///
/// # Returns
///
/// Returns the result of calling the provided closure with the global registry.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::registry::with_global_registry;
/// let namespace = with_global_registry(|registry| {
///     registry.namespace().map(|s| s.to_owned())
/// });
/// ```
pub fn with_global_registry<F, R>(f: F) -> R
where
    F: FnOnce(&Registry) -> R,
{
    let provider = registry_provider();
    let registry = provider.get().read();
    f(&registry)
}

/// Executes a function with mutable access to the global registry.
///
/// This function provides safe, mutable access to the global registry without
/// exposing the underlying synchronization primitives. The global registry will
/// be initialized with default settings if it hasn't been set previously.
///
/// # Arguments
///
/// * `f` - A closure that takes a mutable reference to the [`Registry`] and returns a value of type
///   `R`
///
/// # Returns
///
/// Returns the result of calling the provided closure with the global registry.
///
/// # Example
///
/// ```rust
/// # use fastmetrics::{
/// #     registry::with_global_registry_mut,
/// #     metrics::counter::Counter
/// # };
/// let res = with_global_registry_mut(|registry| {
///     // Perform mutable operations on the registry
///     registry.register("my_counter", "my_counter help", <Counter>::default()).map(|_| ())
/// });
/// ```
pub fn with_global_registry_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut Registry) -> R,
{
    let provider = registry_provider();
    let mut registry = provider.get().write();
    f(&mut registry)
}

/// Registers a metric to the global [`Registry`] and returns the metric instance.
///
/// This function provides a convenient way to register metrics with the global registry
/// while retaining ownership of the metric for updates. It's particularly useful with
/// `LazyLock` for creating static global metrics.
///
/// # Arguments
///
/// * `name` - The name of the metric (must be in `snake_case` format)
/// * `help` - A description of what the metric measures
/// * `metric` - The metric instance to register (must implement [`Clone`])
///
/// # Returns
///
/// Returns `Ok(metric)` if registration succeeds, or [`RegistryError`] if:
/// - A same metric already exists
/// - The metric name is not in `snake_case` format
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::counter::Counter,
/// #     registry::{register, RegistryError},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// let counter = register("http_requests_total", "Total HTTP requests", <Counter>::default())?;
///
/// // Use the returned counter
/// counter.inc();
/// assert_eq!(counter.total(), 1);
/// # Ok(())
/// # }
/// ```
///
/// ## With LazyLock for static metrics
///
/// ```rust
/// # use std::sync::LazyLock;
/// # use fastmetrics::{
/// #     metrics::counter::Counter,
/// #     registry::register,
/// # };
/// static REQUEST_COUNTER: LazyLock<Counter> = LazyLock::new(|| {
///     register("requests_total", "Total requests processed", <Counter>::default())
///         .expect("Failed to register counter")
/// });
///
/// fn handle_request() {
///     REQUEST_COUNTER.inc();
/// }
/// ```
///
/// ## Error handling
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::counter::Counter,
/// #     registry::{register, RegistryError},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// // Register first counter
/// let counter1 = register("my_counter", "A counter", <Counter>::default())?;
///
/// // Try to register another counter with the same name - this will fail
/// let result = register("my_counter", "Another counter", <Counter>::default());
/// assert!(matches!(result, Err(RegistryError::AlreadyExists { .. })));
/// # Ok(())
/// # }
/// ```
pub fn register<M>(
    name: impl Into<Cow<'static, str>>,
    help: impl Into<Cow<'static, str>>,
    metric: M,
) -> Result<M, RegistryError>
where
    M: Metric + Clone + 'static,
{
    register_metric(name, help, None::<Unit>, metric)
}

/// Registers a metric with a unit to the global [`Registry`] and returns the metric instance.
///
/// This function is similar to [`register`] but allows specifying a unit for the metric,
/// which is important for proper metric interpretation and display in monitoring systems.
///
/// # Arguments
///
/// * `name` - The name of the metric (must be in `snake_case` format)
/// * `help` - A description of what the metric measures
/// * `unit` - The unit of measurement (e.g., [`Unit::Seconds`], [`Unit::Bytes`])
/// * `metric` - The metric instance to register (must implement [`Clone`])
///
/// # Returns
///
/// Returns `Ok(metric)` if registration succeeds, or [`RegistryError`] if:
/// - A same metric already exists
/// - The metric name is not in `snake_case` format
/// - The unit format is invalid (custom units must be in `lowercase` format)
/// - The metric type doesn't support units (StateSet, Info, Unknown types must have empty units)
///
/// # Examples
///
/// ## With predefined units
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::histogram::Histogram,
/// #     registry::{register_with_unit, RegistryError, Unit},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// let duration_histogram = register_with_unit(
///     "request_duration",
///     "HTTP request duration",
///     Unit::Seconds,
///     Histogram::default(),
/// )?;
///
/// duration_histogram.observe(0.1); // 100ms
/// # Ok(())
/// # }
/// ```
///
/// ## With custom units
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::gauge::Gauge,
/// #     registry::{register_with_unit, RegistryError},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// let temperature = register_with_unit(
///     "cpu_temperature",
///     "CPU temperature",
///     "celsius",
///     <Gauge>::default(),
/// )?;
///
/// temperature.set(65);
/// # Ok(())
/// # }
/// ```
///
/// ## With LazyLock for static metrics
///
/// ```rust
/// # use std::sync::LazyLock;
/// # use fastmetrics::{
/// #     metrics::gauge::Gauge,
/// #     registry::{register_with_unit, Unit},
/// # };
/// static MEMORY_USAGE: LazyLock<Gauge> = LazyLock::new(|| {
///     register_with_unit(
///         "memory_usage",
///         "Current memory usage",
///         Unit::Bytes,
///         <Gauge>::default(),
///     )
///     .expect("Failed to register memory_usage gauge")
/// });
///
/// fn update_memory_stats() {
///     MEMORY_USAGE.set(1024 * 1024 * 512); // 512MB
/// }
/// ```
///
/// ## Error cases
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::gauge::Gauge,
/// #     registry::{register_with_unit, RegistryError},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// // Invalid unit format (must be in lowercase format)
/// let result = register_with_unit(
///     "invalid_metric",
///     "Invalid metric",
///     "UPPERCASE",
///     <Gauge>::default(),
/// );
/// assert!(matches!(result, Err(RegistryError::OtherUnitFormatMustBeLowercase { .. })));
/// # Ok(())
/// # }
/// ```
pub fn register_with_unit<M>(
    name: impl Into<Cow<'static, str>>,
    help: impl Into<Cow<'static, str>>,
    unit: impl Into<Unit>,
    metric: M,
) -> Result<M, RegistryError>
where
    M: Metric + Clone + 'static,
{
    register_metric(name, help, Some(unit), metric)
}

/// Registers a metric with an optional unit to the global [`Registry`] and returns the metric
/// instance.
///
/// This is the most flexible registration method that allows specifying an optional unit.
/// It serves as the underlying implementation for both [`register`] and [`register_with_unit`].
/// Use [`register`] for metrics without units or [`register_with_unit`] for metrics with units
/// unless you need the flexibility of optional units.
///
/// # Arguments
///
/// * `name` - The name of the metric (must be in `snake_case` format)
/// * `help` - A description of what the metric measures
/// * `unit` - An optional unit of measurement (e.g., [`Some(Unit::Seconds)`], [`None::<Unit>`])
/// * `metric` - The metric instance to register (must implement [`Clone`])
///
/// # Returns
///
/// Returns `Ok(metric)` if registration succeeds, or [`RegistryError`] if:
/// - A same metric already exists
/// - The metric name is not in `snake_case` format
/// - The unit format is invalid (custom units must be in `lowercase` format)
/// - The metric type doesn't support units (StateSet, Info, Unknown types must have empty units)
///
/// # Examples
///
/// ## Register without the unit
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::counter::Counter,
/// #     registry::{register_metric, RegistryError, Unit},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// let counter = register_metric(
///     "requests_total",
///     "Total number of requests",
///     None::<Unit>,
///     <Counter>::default(),
/// )?;
///
/// counter.inc();
/// assert_eq!(counter.total(), 1);
/// # Ok(())
/// # }
/// ```
///
/// ## Register with a unit
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::histogram::Histogram,
/// #     registry::{register_metric, RegistryError, Unit},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// let histogram = register_metric(
///     "http_request_duration",
///     "Duration of HTTP request",
///     Some(Unit::Seconds),
///     Histogram::default(),
/// )?;
///
/// histogram.observe(0.1); // 100ms
/// # Ok(())
/// # }
/// ```
///
/// ## Conditional unit based on configuration
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::gauge::Gauge,
/// #     registry::{register_metric, RegistryError, Unit},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// let use_metric_units = true; // from config
/// let unit: Option<Unit> = if use_metric_units {
///     Some("celsius".into())
/// } else {
///     None
/// };
///
/// let temperature = register_metric(
///     "cpu_temperature",
///     "CPU temperature",
///     unit,
///     <Gauge>::default(),
/// )?;
///
/// temperature.set(65);
/// # Ok(())
/// # }
/// ```
///
/// ## With LazyLock for static metrics
///
/// ```rust
/// # use std::sync::LazyLock;
/// # use fastmetrics::{
/// #     metrics::gauge::Gauge,
/// #     registry::{register_metric, Unit},
/// # };
/// static MEMORY_USAGE: LazyLock<Gauge> = LazyLock::new(|| {
///     register_metric(
///         "memory_usage",
///         "Current memory usage",
///         Some(Unit::Bytes),
///         <Gauge>::default(),
///     )
///     .expect("Failed to register memory_usage gauge")
/// });
///
/// fn update_memory_stats() {
///     MEMORY_USAGE.set(1024 * 1024 * 512); // 512MB
/// }
/// ```
///
/// ## Error cases
///
/// ```rust
/// # use fastmetrics::{
/// #     metrics::{counter::Counter, gauge::Gauge},
/// #     registry::{register_metric, RegistryError, Unit},
/// # };
/// # fn main() -> Result<(), RegistryError> {
/// // Invalid unit format (must be lowercase)
/// let result = register_metric(
///     "invalid_metric",
///     "Invalid metric",
///     Some("UPPERCASE"),
///     <Gauge>::default(),
/// );
/// assert!(matches!(result, Err(RegistryError::OtherUnitFormatMustBeLowercase { .. })));
///
/// // Duplicate registration
/// let counter1 = register_metric("my_counter", "A counter", None::<Unit>, <Counter>::default())?;
/// let result = register_metric("my_counter", "Another counter", None::<Unit>, <Counter>::default());
/// assert!(matches!(result, Err(RegistryError::AlreadyExists { .. })));
/// # Ok(())
/// # }
/// ```
pub fn register_metric<M>(
    name: impl Into<Cow<'static, str>>,
    help: impl Into<Cow<'static, str>>,
    unit: Option<impl Into<Unit>>,
    metric: M,
) -> Result<M, RegistryError>
where
    M: Metric + Clone + 'static,
{
    with_global_registry_mut(|registry| {
        registry.register_metric(name, help, unit, metric.clone()).map(|_| metric)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_test_provider<F, R>(provider: &'static dyn RegistryProvider, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        TEST_REGISTRY.with(|current| {
            let old_provider = current.borrow_mut().replace(provider);
            let result = f();
            *current.borrow_mut() = old_provider;
            result
        })
    }

    #[derive(Default)]
    struct TestRegistry {
        registry: OnceLock<RwLock<Registry>>,
    }

    impl TestRegistry {
        fn new(registry: Registry) -> Self {
            let this = Self { registry: OnceLock::new() };
            let _ = this.registry.set(RwLock::new(registry));
            this
        }
    }

    impl RegistryProvider for TestRegistry {
        fn set(&self, registry: Registry) -> Result<(), SetRegistryError> {
            self.registry.set(RwLock::new(registry)).map_err(|_| SetRegistryError)
        }

        fn get(&self) -> &RwLock<Registry> {
            self.registry.get_or_init(|| RwLock::new(Registry::default()))
        }
    }

    fn create_test_provider(registry: Registry) -> &'static TestRegistry {
        Box::leak(Box::new(TestRegistry::new(registry)))
    }

    fn create_default_test_provider() -> &'static TestRegistry {
        Box::leak(Box::new(TestRegistry::default()))
    }

    #[test]
    fn test_global_registry() {
        let provider = create_default_test_provider();
        with_test_provider(provider, || {
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), None);
            });
        });

        let registry = Registry::builder().with_namespace("test1").build();
        let provider = create_test_provider(registry);
        with_test_provider(provider, || {
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), Some("test1"));
            });
        });

        let registry = Registry::builder().with_namespace("test2").build();
        let provider = create_test_provider(registry);
        with_test_provider(provider, || {
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), Some("test2"));
            });
        });
    }

    #[test]
    fn test_concurrent_access() {
        let registry = Registry::builder().with_namespace("concurrent").build();
        let provider = create_test_provider(registry);

        with_test_provider(provider, || {
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), Some("concurrent"));
            });

            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let test_provider = provider;
                    std::thread::spawn(move || {
                        // use the provider directly instead of thread_local
                        let registry = test_provider.get().read();
                        registry.namespace().map(|s| s.to_owned())
                    })
                })
                .collect();

            for handle in handles {
                let namespace = handle.join().expect("Thread should not panic");
                assert_eq!(namespace, Some("concurrent".to_owned()));
            }
        });
    }

    #[test]
    fn test_mutable_and_immutable_access() {
        let registry = Registry::builder().with_namespace("test").build();
        let provider = create_test_provider(registry);

        with_test_provider(provider, || {
            // access the mutable registry
            with_global_registry_mut(|registry| {
                assert_eq!(registry.namespace(), Some("test"));
                // do mutable operations, such as registering metrics
            });

            // access the immutable registry
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), Some("test"));
            });
        });
    }

    #[test]
    fn duplicated_set_global_registry() {
        // Create a test provider to isolate this test
        let provider = create_default_test_provider();

        with_test_provider(provider, || {
            // The First call should succeed
            let registry1 = Registry::builder().with_namespace("first").build();
            let result1 = set_global_registry(registry1);
            assert!(result1.is_ok(), "First set_global_registry should succeed");

            // Verify the registry was set
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), Some("first"));
            });

            // The Second call should fail since the global registry is already initialized
            let registry2 = Registry::builder().with_namespace("second").build();
            let result2 = set_global_registry(registry2);
            assert!(result2.is_err(), "Second set_global_registry should fail");

            // Verify the original registry is still in place
            with_global_registry(|registry| {
                assert_eq!(registry.namespace(), Some("first"));
            });
        });
    }
}
