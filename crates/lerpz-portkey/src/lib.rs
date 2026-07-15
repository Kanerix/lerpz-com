//! Helpers for working with the [Portkey](https://portkey.ai) AI gateway.
//!
//! Portkey forwards provider responses (and errors) largely untouched, so
//! upstreams don't share a single error schema. This crate collapses whatever
//! shape an error arrives in into a single human-readable line that is safe to
//! surface to end users.

mod error;

pub use error::humanize_error;
