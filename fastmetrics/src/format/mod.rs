//! Wire formats.
//!
//! This module contains all metric exposition backends used by FastMetrics.
//!
//! ## Module availability
//!
//! - [`text`] is always available.
//! - `prost` module is available when feature `prost` is enabled.
//! - `protobuf` module is available when feature `protobuf` is enabled.
//!
//! ## Text format
//!
//! The [`text`] module exposes [`text::encode`] and [`text::encode_with`].
//!
//! It follows the [OpenMetrics text format] and supports the [Prometheus text format]:
//!
//! - `TextProfile::OpenMetrics1` (default)
//! - `TextProfile::Prometheus004`
//!
//! ## Protobuf format
//!
//! Protobuf support is feature-gated and available through two interchangeable modules:
//! - [`prost`] (feature `prost`)
//! - [`protobuf`] (feature `protobuf`)
//!
//! Both modules expose:
//! - [`prost::encode`] / [`prost::encode_with`]
//! - [`protobuf::encode`] / [`protobuf::encode_with`]
//!
//! Both modules re-export the same profile type:
//! - [`prost::ProtobufProfile`]
//! - [`protobuf::ProtobufProfile`]
//!
//! Protobuf profiles:
//! - `Prometheus` (default): a length-delimited stream of `io.prometheus.client.MetricFamily`.
//! - `OpenMetrics1`: a single `openmetrics.MetricSet` message.
//!
//! References:
//! - [Prometheus protobuf format], [Prometheus protobuf schema]
//! - [OpenMetrics protobuf format], [OpenMetrics protobuf schema]
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
