//! Metadata definitions for metrics in the OpenMetrics specification.
//!
//! The metadata is used when registering metrics and when exposing them in the OpenMetrics format.
//!
//! The primary structures in this module are:
//! - [`Metadata`]: Contains the core metadata for a metric family (name, help, type, unit)
//! - [`Unit`]: Represents standard measurement units according to the OpenMetrics specification

use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use crate::raw::MetricType;

/// The metadata of a metric family.
///
/// There are four pieces of metadata: name, TYPE, UNIT and HELP.
#[derive(Clone, Debug)]
pub struct Metadata {
    name: Cow<'static, str>,
    help: Cow<'static, str>,
    ty: MetricType,
    unit: Option<Unit>,
}

impl PartialEq for Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.ty == other.ty && self.unit == other.unit
    }
}

impl Eq for Metadata {}

impl Hash for Metadata {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.ty.hash(state);
        self.unit.hash(state);
    }
}

impl Metadata {
    /// Creates a new [`Metadata`] of metric family.
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        help: impl Into<Cow<'static, str>>,
        ty: MetricType,
        unit: Option<Unit>,
    ) -> Self {
        Self { name: name.into(), help: help.into(), ty, unit }
    }

    /// Returns the name of the metric family.
    ///
    /// The name uniquely identifies the metric family in the registry and
    /// is used when exposing metrics in the OpenMetrics format.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the help text of the metric family.
    ///
    /// The help text provides a description of what the metric measures and
    /// is included in the OpenMetrics output as a HELP comment.
    pub fn help(&self) -> &str {
        self.help.as_ref()
    }

    /// Returns the type of the metric family.
    ///
    /// The type indicates what kind of metric this is (Counter, Gauge, etc.) and
    /// is included in the OpenMetrics output as a TYPE comment.
    pub fn metric_type(&self) -> MetricType {
        self.ty
    }

    /// Returns the optional unit of the metric family.
    ///
    /// The unit specifies the measurement unit for the metric values (e.g., seconds, bytes).
    /// If present, it is included in the OpenMetrics output as part of the metric name.
    pub fn unit(&self) -> Option<&Unit> {
        self.unit.as_ref()
    }
}

/// The standard measurement units according to the [OpenMetrics specification].
///
/// [OpenMetrics specification]: https://github.com/prometheus/OpenMetrics/blob/main/specification/OpenMetrics.md#units-and-base-units
#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Unit {
    Seconds,
    Bytes,
    Joules,
    Grams,
    Meters,
    Ratios,
    Volts,
    Amperes,
    Celsius,
    Other(Cow<'static, str>),
}

impl Unit {
    /// Returns the string representation for the specified metric unit.
    pub fn as_str(&self) -> &str {
        match self {
            Unit::Seconds => "seconds",
            Unit::Bytes => "bytes",
            Unit::Joules => "joules",
            Unit::Grams => "grams",
            Unit::Meters => "meters",
            Unit::Ratios => "ratios",
            Unit::Volts => "volts",
            Unit::Amperes => "amperes",
            Unit::Celsius => "celsius",
            Unit::Other(other) => other.as_ref(),
        }
    }
}
