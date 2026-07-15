//! Helpers for working with the [Portkey](https://portkey.ai) AI gateway.
//!
//! Portkey forwards provider responses (and errors) largely untouched, so
//! upstreams don't share a single error schema. [`humanize_error`] collapses
//! whatever shape an error arrives in into a single human-readable line that is
//! safe to surface to end users.
//!
//! Two optional, feature-gated integrations are also provided for wiring the
//! gateway into an AI client:
//!
//! - `async-openai`: [`PortkeyConfig`], an [`async-openai`](async_openai)
//!   `Config` that routes requests through Portkey.
//! - `rig`: [`build_client`], which builds a [`rig-core`](rig_core) client
//!   pointed at the gateway.

mod error;

pub use error::humanize_error;

#[cfg(feature = "async-openai")]
mod config;

#[cfg(feature = "async-openai")]
pub use config::PortkeyConfig;

#[cfg(feature = "rig")]
mod client;

#[cfg(feature = "rig")]
pub use client::build_client;
