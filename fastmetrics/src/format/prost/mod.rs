//! Prometheus protobuf exposition format using [prost](https://github.com/tokio-rs/prost) crate.

mod prometheus;

pub use super::profile::ProtobufProfile;
use crate::{error::Result, registry::Registry};

/// Data models that are automatically generated from [Prometheus protobuf schema].
///
/// [Prometheus protobuf schema]: https://github.com/prometheus/client_model/blob/master/io/prometheus/client/metrics.proto
#[allow(missing_docs)]
#[allow(clippy::all)]
mod prometheus_data_model {
    include!(concat!(env!("OUT_DIR"), "/prost/io.prometheus.client.rs"));
}

/// Encodes metrics from a registry into protobuf format with an explicit profile.
pub fn encode(
    buffer: &mut impl prost::bytes::BufMut,
    registry: &Registry,
    profile: ProtobufProfile,
) -> Result<()> {
    encode_with(buffer, registry, profile, crate::metrics::lazy_group::enter_scope)
}

/// Encodes metrics in protobuf format with explicit profile and scope hook.
pub fn encode_with<G>(
    buffer: &mut impl prost::bytes::BufMut,
    registry: &Registry,
    profile: ProtobufProfile,
    enter_scope: impl FnOnce() -> G,
) -> Result<()> {
    // The returned value is kept alive for the duration of encoding and then dropped.
    let _guard = enter_scope();

    match profile {
        ProtobufProfile::Prometheus => prometheus::encode(buffer, registry),
    }
}
