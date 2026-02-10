//! Wire format encoders.
//!
//! This module groups all metric exposition backends used by FastMetrics.
//!
//! ## Availability
//!
//! - [`text`] is always available.
//! - [`prost`] is available with feature `prost`.
//! - [`protobuf`] is available with feature `protobuf`.
//!
//! ## Text format
//!
//! The [`text`] module exposes the API:
//! - `encode(buffer, registry, profile)`
//! - `encode_with(buffer, registry, profile, enter_scope)`.
//!
//! Text profiles:
//! - `OpenMetrics1` (default): [OpenMetrics text format]
//! - `Prometheus004`: [Prometheus text format]
//!
//! ## Protobuf format
//!
//! Protobuf support is feature-gated and provided by two interchangeable backends:
//! - [`prost`] (feature `prost`)
//! - [`protobuf`] (feature `protobuf`)
//!
//! Both backends expose the same API shape:
//! - `encode(buffer, registry, profile)`
//! - `encode_with(buffer, registry, profile, enter_scope)`
//!
//! Protobuf profiles:
//! - `Prometheus` (default): length-delimited `io.prometheus.client.MetricFamily` stream
//!   - [Prometheus protobuf format]
//!   - [Prometheus protobuf schema]
//! - `OpenMetrics1`: single `openmetrics.MetricSet` message
//!   - [OpenMetrics protobuf format]
//!   - [OpenMetrics protobuf schema]
//!
//! [OpenMetrics text format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#text-format
//! [Prometheus text format]: https://prometheus.io/docs/instrumenting/exposition_formats/#text-format-details
//! [OpenMetrics protobuf format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#protobuf-format
//! [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto
//! [Prometheus protobuf format]: https://prometheus.io/docs/instrumenting/exposition_formats/#protobuf-format
//! [Prometheus protobuf schema]: https://github.com/prometheus/client_model/blob/master/io/prometheus/client/metrics.proto

mod profile;

#[cfg(feature = "prost")]
pub mod prost;
#[cfg(feature = "protobuf")]
pub mod protobuf;
pub mod text;
