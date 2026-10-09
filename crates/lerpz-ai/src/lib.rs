//! Shared AI helpers for Lerpz services.
//!
//! The [`generation`] module defines shared requests, events and generation
//! traits. Enable `portkey` for [Portkey](https://portkey.ai) gateway
//! implementations, or `rig` for its `rig-core` client integration.

#[cfg(feature = "portkey")]
pub mod portkey;

pub mod generation;
