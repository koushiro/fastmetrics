//! This crate provides some derive macros for `fastmetrics`.

#![deny(missing_docs)]
#![deny(unsafe_code)]
// False positive: https://github.com/rust-lang/rust/issues/129637
// #![deny(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod encode_label_set;
mod encode_label_value;
mod label_attributes;
mod label_set_schema;
mod register;
mod state_set_value;
mod utils;

use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

/// Derive the `EncodeLabelSet` trait for structs.
///
/// This macro automatically implements the `EncodeLabelSet` trait,
/// which allows the struct to be used as a set of metric labels.
/// This is useful for creating structured label sets that can be attached to metrics.
///
/// # Example
///
/// ```rust
/// # use fastmetrics_derive::{EncodeLabelSet, EncodeLabelValue};
/// #[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
/// struct Labels {
///    #[label(rename = "op")]
///    operation: Operation,
///    error: Option<Error>,
///
///    #[label(flatten)]
///    extra: ExtraLabels,
///
///    #[label(skip)]
///    _skip: u64,
/// }
///
/// #[derive(Clone, Eq, PartialEq, Hash, EncodeLabelSet)]
/// struct ExtraLabels {
///    region: &'static str,
/// }
///
/// #[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
/// enum Operation {
///    Read,
///    Write,
///    List,
///    Delete,
/// }
///
/// #[derive(Clone, Eq, PartialEq, Hash, EncodeLabelValue)]
/// enum Error {
///    NotFound,
///    Fail,
/// }
/// ```
#[proc_macro_derive(EncodeLabelSet, attributes(label))]
pub fn derive_encode_label_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    encode_label_set::expand_derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive the `LabelSetSchema` trait for structs.
///
/// This macro automatically implements the `LabelSetSchema` trait, which
/// allows label set schema to be generated.
///
/// # Example
///
/// ```rust
/// # use fastmetrics_derive::LabelSetSchema;
/// #[derive(Clone, Eq, PartialEq, Hash, LabelSetSchema)]
/// struct HttpLabels {
///    #[label(rename = "op")]
///    operation: Operation,
///    error: Option<Error>,
///
///    #[label(flatten)]
///    extra: ExtraLabels,
///
///    #[label(skip)]
///    _skip: u64,
/// }
///
/// #[derive(Clone, Eq, PartialEq, Hash, LabelSetSchema)]
/// struct ExtraLabels {
///    region: &'static str,
/// }
///
/// #[derive(Clone, Eq, PartialEq, Hash)]
/// enum Operation {
///    Read,
///    Write,
///    List,
///    Delete,
/// }
///
/// #[derive(Clone, Eq, PartialEq, Hash)]
/// enum Error {
///    NotFound,
///    Fail,
/// }
/// ```
#[proc_macro_derive(LabelSetSchema, attributes(label))]
pub fn derive_label_set_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    label_set_schema::expand_derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive the `EncodeLabelValue` trait for enums or single-field tuple structs.
///
/// This macro generates an implementation of the `EncodeLabelValue` trait,
/// which allows values to be used as metric label values.
///
/// # Example
///
/// ```rust
/// # use fastmetrics_derive::EncodeLabelValue;
/// #[derive(EncodeLabelValue)]
/// enum Status {
///     Success,
///     Error,
///     Pending,
/// }
///
/// #[derive(EncodeLabelValue)]
/// struct HttpStatus(u16);
/// ```
#[proc_macro_derive(EncodeLabelValue)]
pub fn derive_encode_label_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    encode_label_value::expand_derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive the `StateSetValue` trait for enums.
///
/// This macro implements the `StateSetValue` trait, which allows an enum
/// to be used as a value in a state set metric.
///
/// # Example
///
/// ```rust
/// # use fastmetrics_derive::StateSetValue;
/// #[derive(PartialEq, StateSetValue)]
/// enum ServiceState {
///     Available,
///     Degraded,
///     Down,
/// }
/// ```
#[proc_macro_derive(StateSetValue)]
pub fn derive_state_set_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    state_set_value::expand_derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Derive the `Register` trait for structs.
///
/// This macro implements automatic registration of metrics with a registry.
/// It processes `#[register]` attributes on struct fields to configure how
/// each metric should be registered.
///
/// # Example
///
/// ```rust
/// # use std::marker::PhantomData;
/// # use fastmetrics::{
/// #     format::text::{self, TextProfile},
/// #     metrics::{counter::Counter, gauge::Gauge, histogram::Histogram, family::Family},
/// #     registry::{Register, Registry},
/// # };
/// static OVERRIDE_HELP: &str = "Custom help text that overrides doc comments";
///
/// #[derive(Default, fastmetrics_derive::Register)]
/// struct DemoMetrics {
///     /// My counter help
///     #[register(rename = "my_counter")]
///     counter_family: Family<(), Counter>,
///
///     /// My gauge help line 1
///     /// help line 2
///     /// help line 3
///     #[register(rename = "my_gauge")]
///     gauge: Gauge,
///
///     // No help
///     #[register(unit(Bytes))]
///     counter: Counter,
///
///     /// This doc comment will be ignored
///     #[register(help = OVERRIDE_HELP)]
///     override_help_counter: Counter,
///
///     /**
///
///     My histogram help line 1
///
///     help line 2
///     help line 3
///
///     */
///     #[register(rename = "my_histogram", unit = "bytes")]
///     histogram: Histogram,
///
///     #[register(subsystem = "inner")]
///     inner: InnerMetrics,
///
///     #[register(flatten)]
///     flatten: FlattenMetrics,
///
///     // skip the field
///     #[register(skip)]
///     _skip: (),
/// }
///
/// #[derive(Default, fastmetrics_derive::Register)]
/// struct InnerMetrics {
///     /// Inner counter help
///     counter: Counter,
/// }
///
/// #[derive(Default, fastmetrics_derive::Register)]
/// struct FlattenMetrics {
///     /// Flatten gauge help
///     flatten_gauge: Gauge,
/// }
///
/// let mut registry = Registry::builder().with_namespace("demo").build().unwrap();
///
/// let metrics = DemoMetrics::default();
/// metrics.register(&mut registry).unwrap();
///
/// let mut output = String::new();
/// text::encode(&mut output, &registry, TextProfile::default()).unwrap();
/// // println!("{}", output);
/// ```
#[proc_macro_derive(Register, attributes(register))]
pub fn derive_register_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    register::expand_derive(input).unwrap_or_else(Error::into_compile_error).into()
}
