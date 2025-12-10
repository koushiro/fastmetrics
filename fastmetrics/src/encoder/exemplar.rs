use std::time::Duration;

use crate::{encoder::EncodeLabelSet, error::Result};

/// Trait for encoding an exemplar.
pub trait ExemplarEncoder {
    /// Encodes an exemplar with its associated labels, value, and optional timestamp.
    ///
    /// # Arguments
    ///
    /// * `label_set` - The set of labels associated with this exemplar
    /// * `value` - The observed value of the exemplar
    /// * `timestamp` - Optional timestamp when the exemplar was recorded
    fn encode(
        &mut self,
        label_set: &dyn EncodeLabelSet,
        value: f64,
        timestamp: Option<Duration>,
    ) -> Result<()>;
}

/// Trait for types that can be encoded as an exemplar.
pub trait EncodeExemplar {
    /// Encodes this exemplar using the provided [`ExemplarEncoder`].
    fn encode(&self, encoder: &mut dyn ExemplarEncoder) -> Result<()>;
}

impl EncodeExemplar for () {
    fn encode(&self, _encoder: &mut dyn ExemplarEncoder) -> Result<()> {
        Ok(())
    }
}
