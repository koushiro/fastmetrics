//! [Open Metrics Summary](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#summary) metric type.
//!
//! This metric is intentionally left unimplemented because summaries require client-side
//! quantile estimation, stateful streams per label set, and export payloads that cannot
//! be merged server-side. These requirements conflict with this crate’s design goals:
//! keeping collectors zero-allocation in the hot path, offloading aggregation to the backend,
//! and favoring histograms for percentile analysis.
//!
//! Downstream users who still need Summary semantics can implement their own type by adhering
//! to the same metric traits used by other built-in metric types.
//! This allows them to reuse this crate’s encoders without forking or duplicating logic.
