//! Wire format encoders.
//!
//! This module groups all metric exposition backends used by FastMetrics.
//!
//! ## Availability
//!
//! - [`text`] is always available.
//! - [`protobuf`] is available with feature `protobuf`.
//!
//! ## Text format
//!
//! The [`text`] module exposes the API:
//! - `encode(buffer, registry, profile)`
//! - `encode_with(buffer, registry, profile, enter_scope)`.
//!
//! Text profiles:
//! - `PrometheusV0_0_4`: [Prometheus text format]
//! - `PrometheusV1_0_0`: [Prometheus text format] + escaping scheme support
//! - `OpenMetricsV0_0_1`: [OpenMetrics text format]
//! - `OpenMetricsV1_0_0` (default): [OpenMetrics text format] + escaping scheme support
//!
//! ## Protobuf format
//!
//! Protobuf support is provided by [`protobuf`] with feature `protobuf`.
//!
//! The [`protobuf`] module exposes the API:
//! - `encode(buffer, registry, profile)`
//! - `encode_with(buffer, registry, profile, enter_scope)`
//!
//! The backend supports `ProtobufProfile::Prometheus`, which encodes a length-delimited
//! `io.prometheus.client.MetricFamily` stream:
//! - [Prometheus protobuf format]
//! - [Prometheus protobuf schema]
//!
//! [OpenMetrics text format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#text-format
//! [Prometheus text format]: https://prometheus.io/docs/instrumenting/exposition_formats/#text-format-details
//! [Prometheus protobuf format]: https://prometheus.io/docs/instrumenting/exposition_formats/#protobuf-format
//! [Prometheus protobuf schema]: https://github.com/prometheus/client_model/blob/master/io/prometheus/client/metrics.proto

mod profile;

#[cfg(feature = "protobuf")]
pub mod protobuf;
pub mod text;
