#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", env!("CARGO_PKG_README")))]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Enforce platform requirements without interfering with crate-level inner docs (`//!`).
// This must live after the crate docs block.
#[cfg(not(target_has_atomic = "64"))]
compile_error!("fastmetrics requires 64-bit atomic support (target_has_atomic = \"64\").");

#[cfg(feature = "derive")]
pub use fastmetrics_derive as derive;

pub mod encoder;
pub mod error;
pub mod format;
pub mod metrics;
pub mod raw;
pub mod registry;
