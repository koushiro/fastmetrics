//! This crate provides some derive macros for `fastmetrics`.

#![deny(missing_docs)]
#![deny(unsafe_code)]
// False positive: https://github.com/rust-lang/rust/issues/129637
// #![deny(unused_crate_dependencies)]

mod encode_label_set;
mod encode_label_value;
mod register;
mod state_set_value;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

/// Derive the `EncodeLabelSet` trait for structs.
///
/// This macro automatically implements the `EncodeLabelSet` trait,
/// which allows the struct to be used as a set of metric labels.
/// This is useful for creating structured label sets that can be attached to metrics.
///
/// # Example
///
/// ```rust
/// # use fastmetrics_derive::EncodeLabelSet;
/// #[derive(EncodeLabelSet)]
/// struct MyLabels {
///     service: String,
///     endpoint: String,
/// }
/// ```
#[proc_macro_derive(EncodeLabelSet)]
pub fn derive_encode_label_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    encode_label_set::expand_derive(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

// Derive the `EncodeLabelValue` trait for enums.
///
/// This macro generates an implementation of the `EncodeLabelValue` trait,
/// which allows them to be used as values in metric labels.
/// This is useful for ensuring type safety when using enumerated values as labels.
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
/// #    format::text,
/// #     metrics::{counter::Counter, histogram::Histogram},
/// #     registry::{Register, Registry},
/// # };
/// #[derive(Default, fastmetrics_derive::Register)]
/// struct Metrics {
///     /// Total HTTP requests
///     http_requests: Counter,
///
///     /// Duration of HTTP requests
///     #[register(rename = "http_request_duration", unit(Seconds))]
///     request_duration: Histogram,
///
///     #[register(subsystem = "inner")]
///     inner: InnerMetrics,
///
///     #[register(flatten)]
///     flatten: FlattenMetrics,
///
///     #[register(skip)]
///     _skip: ()
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
///     /// Flatten counter help
///     flatten_counter: Counter,
/// }
///
/// let mut registry = Registry::default();
///
/// let metrics = Metrics::default();
/// metrics.register(&mut registry).unwrap();
///
/// let mut output = String::new();
/// text::encode(&mut output, &registry).unwrap();
/// // println!("{}", output);
/// ```
#[proc_macro_derive(Register, attributes(register))]
pub fn derive_register_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    register::expand_derive(input).unwrap_or_else(Error::into_compile_error).into()
}
