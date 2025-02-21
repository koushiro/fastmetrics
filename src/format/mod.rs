//! Wire formats
//!
//! This module provides implementations of different metric exposition formats.
//!
//! Supported formats:
//!
//! # Text format
//!
//! [OpenMetrics text format] MUST be supported and is the default.
//!
//! [OpenMetrics text format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#text-format
//!
//! # Protobuf format
//!
//! [OpenMetrics protobuf format] MUST follow the proto3 version of the protocol buffer language and
//! all payloads MUST be a single binary encoded MetricSet message, as defined by the [OpenMetrics
//! protobuf schema].
//!
//! [OpenMetrics protobuf format]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#protobuf-format
//! [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto

#[cfg(feature = "protobuf")]
pub mod protobuf;
pub mod text;
