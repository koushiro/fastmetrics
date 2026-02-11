//! Text exposition format.

mod config;
mod encoder;
mod names;
#[cfg(test)]
mod tests;

use std::fmt;

pub use super::profile::{EscapingScheme, TextProfile};
use crate::{error::Result, registry::Registry};

/// Encodes metrics from a [`Registry`] into text format with an explicit profile.
///
/// This is the default text-encoding entrypoint for most users.
/// It installs the standard scrape scope hook ([`crate::metrics::lazy_group::enter_scope`]) so
/// grouped lazy metrics can use scrape-scoped caching.
pub fn encode(
    writer: &mut impl fmt::Write,
    registry: &Registry,
    profile: TextProfile,
) -> Result<()> {
    encode_with(writer, registry, profile, crate::metrics::lazy_group::enter_scope)
}

/// Encodes metrics from a [`Registry`] into text format with explicit profile and scope hook.
///
/// This is the advanced text-encoding entrypoint. The [`encode`] helper is a thin wrapper around
/// this function:
///
/// - [`encode`] = `encode_with(..., lazy_group::enter_scope)`
///
/// The `enter_scope` closure runs once before encoding starts. Its return value is kept alive for
/// the entire encoding pass and then dropped. This is used by grouped lazy metrics for
/// scrape-scoped caching.
///
/// ## Scrape-scoped caching (grouped lazy metrics)
///
/// Grouped lazy metrics created via [`crate::metrics::lazy_group::LazyGroup`] can share an
/// expensive sampling operation within a single scrape.
///
/// To enable this behavior, pass a closure that enters a scrape scope for the full encoding pass.
/// The default wrapper [`encode`] already installs this scope hook internally.
/// If you need the same default hook explicitly, use [`crate::metrics::lazy_group::enter_scope`].
///
/// # Arguments
///
/// - `writer`: Output destination implementing [`fmt::Write`].
/// - `registry`: Source registry.
/// - `profile`: Text format profile selection.
/// - `enter_scope`: Pre-encode scope hook.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an [`Error`](crate::error::Error) if writing fails.
///
/// # Examples
///
/// ```rust
/// # use fastmetrics::{
/// #     error::Result,
/// #     format::text::{self, TextProfile},
/// #     metrics::counter::Counter,
/// #     registry::Registry,
/// # };
/// #
/// # fn main() -> Result<()> {
/// let mut registry = Registry::default();
///
/// // Register a counter
/// let requests = <Counter>::default();
/// registry.register(
///     "http_requests_total",
///     "Total number of HTTP requests",
///     requests.clone()
/// )?;
/// // Update a counter
/// requests.inc();
///
/// // Encode metrics in Prometheus text format
/// let mut output = String::new();
/// // Prometheus 0.0.4 profile, no additional scope:
/// text::encode_with(&mut output, &registry, TextProfile::PrometheusV0_0_4, || ())?;
///
/// // Encode metrics in OpenMetrics text format
/// let mut output = String::new();
/// // OpenMetrics 1.0.0 profile, no additional scope:
/// text::encode_with(
///     &mut output,
///     &registry,
///     TextProfile::default(),
///     || ()
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn encode_with<G>(
    writer: &mut impl fmt::Write,
    registry: &Registry,
    profile: TextProfile,
    enter_scope: impl FnOnce() -> G,
) -> Result<()> {
    // The returned value is kept alive for the duration of encoding and then dropped.
    let _guard = enter_scope();

    encoder::encode(writer, registry, profile.into())
}
