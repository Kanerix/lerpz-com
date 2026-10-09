//! Helpers for working with the [Portkey](https://portkey.ai) AI gateway.
//!
//! Portkey forwards provider responses (and errors) largely untouched, so
//! upstreams don't share a single error schema. [`humanize_error`] collapses
//! whatever shape an error arrives in into a single human-readable line that is
//! safe to surface to end users, and [`classify_error`] additionally reports
//! whether the error was caused by the user's input (e.g. content moderation)
//! so callers can log it at an appropriate level.
//!
//! [`PortkeyConfig`] routes [`async-openai`](async_openai) requests through
//! Portkey. The optional `rig` feature also provides [`build_client`], which
//! builds a [`rig-core`](rig_core) client pointed at the gateway.

mod chat;
mod config;
mod error;
mod image;
mod video;

use crate::generation::Family;
pub use crate::generation::{ErrorKind, UpstreamError, classify_error, humanize_error};
pub use config::PortkeyConfig;
pub use error::{Error, Result};

#[cfg(feature = "rig")]
mod client;

#[cfg(feature = "rig")]
pub use client::build_client;

impl Family {
    /// Resolves a model family name for Portkey routing.
    ///
    /// Unknown or unspecified families use the default behaviour.
    pub fn from_name(name: Option<&str>) -> Self {
        match name.map(str::trim) {
            Some("google") => Self::Google,
            _ => Self::Default,
        }
    }
}
