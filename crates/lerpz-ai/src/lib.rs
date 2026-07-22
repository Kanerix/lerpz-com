//! Shared AI helpers for Lerpz services.
//!
//! Integrations are opt-in via feature flags:
//!
//! - `portkey`: the [`portkey`] module, with helpers for the
//!   [Portkey](https://portkey.ai) AI gateway.
//! - `generation`: the [`generation`] module, with family-aware image, video
//!   and chat generation built on top of the gateway.

#[cfg(feature = "portkey")]
pub mod portkey;

#[cfg(feature = "generation")]
pub mod generation;
