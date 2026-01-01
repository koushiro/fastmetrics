//! Internal module tree for shared metric implementations.
//!
//! This is a private implementation detail and must not be considered stable API.

#[cfg(target_has_atomic = "64")]
pub(crate) mod histogram;
pub(crate) mod lazy;
