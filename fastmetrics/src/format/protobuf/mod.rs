//! Protobuf exposition format using [protobuf](https://github.com/stepancheg/rust-protobuf) crate.

mod openmetrics;
mod prometheus;

use std::io;

pub use super::profile::ProtobufProfile;
use crate::{error::Result, registry::Registry};

/// Data models that are automatically generated from protobuf schemas.
///
/// This module includes generated models for both:
/// - [OpenMetrics protobuf schema]
/// - [Prometheus protobuf schema]
///
/// [OpenMetrics protobuf schema]: https://github.com/prometheus/OpenMetrics/blob/main/proto/openmetrics_data_model.proto
/// [Prometheus protobuf schema]: https://github.com/prometheus/client_model/blob/master/io/prometheus/client/metrics.proto
#[allow(missing_docs)]
#[allow(clippy::all)]
mod generated_data_model {
    include!(concat!(env!("OUT_DIR"), "/protobuf/mod.rs"));
}

/// Encodes metrics from a registry into protobuf format with an explicit profile.
pub fn encode(
    buffer: &mut dyn io::Write,
    registry: &Registry,
    profile: ProtobufProfile,
) -> Result<()> {
    encode_with(buffer, registry, profile, crate::metrics::lazy_group::enter_scope)
}

/// Encodes metrics in protobuf format with explicit profile and scope hook.
pub fn encode_with<G>(
    buffer: &mut dyn io::Write,
    registry: &Registry,
    profile: ProtobufProfile,
    enter_scope: impl FnOnce() -> G,
) -> Result<()> {
    // The returned value is kept alive for the duration of encoding and then dropped.
    let _guard = enter_scope();

    match profile {
        ProtobufProfile::OpenMetrics1 => openmetrics::encode(buffer, registry),
        ProtobufProfile::Prometheus => prometheus::encode(buffer, registry),
    }
}
