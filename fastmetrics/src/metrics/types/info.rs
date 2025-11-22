//! [Open Metrics Info](https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#info) metric type.

use std::fmt;

use crate::{
    encoder::{EncodeLabelSet, EncodeMetric, MetricEncoder},
    raw::{MetricLabelSet, MetricType, TypedMetric},
};

/// Open Metrics [`Info`] metric, which is used to expose textual information which SHOULD NOT
/// change during process lifetime.
#[derive(Clone, Debug)]
pub struct Info<LS> {
    label_set: LS,
}

impl<LS> Info<LS> {
    /// Creates an [`Info`] metric with the given label set.
    pub fn new(label_set: LS) -> Self {
        Self { label_set }
    }

    /// Gets the label set of the [`Info`].
    pub fn get(&self) -> &LS {
        &self.label_set
    }
}

impl<LS> TypedMetric for Info<LS> {
    const TYPE: MetricType = MetricType::Info;
}

impl<LS> MetricLabelSet for Info<LS> {
    type LabelSet = ();
}

impl<LS> EncodeMetric for Info<LS>
where
    LS: EncodeLabelSet + Send + Sync,
{
    fn encode(&self, encoder: &mut dyn MetricEncoder) -> fmt::Result {
        encoder.encode_info(self.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::check_text_encoding;

    #[test]
    fn test_text_encoding() {
        check_text_encoding(
            |registry| {
                let info = Info::new(vec![("version", "1.0")]);
                registry
                    .register("release_version", "My release version", info.clone())
                    .unwrap();
            },
            |output| {
                let expected = indoc::indoc! {r#"
                    # TYPE release_version info
                    # HELP release_version My release version
                    release_version_info{version="1.0"} 1
                    # EOF
                "#};
                assert_eq!(expected, output);
            },
        );
    }
}
