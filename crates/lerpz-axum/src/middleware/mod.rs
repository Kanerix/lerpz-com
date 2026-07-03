//! Reusable [`axum`]/[`tower`] middleware for building HTTP services.
//!
//! Each submodule provides a piece of middleware that can be layered onto a
//! router. Some are gated behind Cargo feature flags:
//!
//! - [`instance`]: captures the request path so [`Problem`] responses can be
//!   populated with an `instance` automatically. Always available.
//! - [`validate`]: request extractors that validate their payload using the
//!   [`validator`] crate. Always available.
//! - [`azure`]: authentication middleware for validating Azure Entra ID access
//!   tokens. Only compiled when the `azure` feature is enabled.
//!
//! [`Problem`]: crate::problem::Problem

pub mod instance;
pub mod validate;

#[cfg(feature = "azure")]
pub mod azure;
