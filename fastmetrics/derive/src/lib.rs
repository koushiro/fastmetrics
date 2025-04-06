//! This crate provides some derive macros for `fastmetrics`.

#![deny(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![forbid(unsafe_code)]

mod encode_label_set;
mod encode_label_value;
mod state_set_value;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

// `fastmetrics::encoder::EncodeLabelSet`
#[proc_macro_derive(EncodeLabelSet)]
pub fn derive_encode_label_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    encode_label_set::expand_derive_encode_label_set(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// `fastmetrics::encoder::EncodeLabelValue`
#[proc_macro_derive(EncodeLabelValue)]
pub fn derive_encode_label_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    encode_label_value::expand_derive_encode_label_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// `fastmetrics::metrics::state_set::StateSetValue`
#[proc_macro_derive(StateSetValue)]
pub fn derive_state_set_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    state_set_value::expand_derive_state_set_value(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
