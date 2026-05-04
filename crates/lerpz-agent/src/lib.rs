//! Common agent infrastructure for Lerpz projects.
//!
//! This crate provides the shared building blocks for constructing
//! LLM-powered agents across all Lerpz services:
//!
//! - [`AgentFactory`] – holds shared infrastructure (LLM client, optional
//!   Qdrant, HTTP client). Built via [`AgentFactoryBuilder`].
//! - [`AgentConfig`] – required core LLM configuration.
//! - [`QdrantConfig`] – optional Qdrant / vector-search configuration,
//!   enabled by calling [`AgentFactoryBuilder::with_qdrant`].
//! - [`portkey`] – builds an OpenAI-compatible [`rig`] client that routes
//!   through the Portkey AI gateway.
//! - [`tools`] – common tool error type and shared tool helpers.
//! - [`error`] – crate-level error type.

pub mod client;
pub mod error;
pub mod factory;

pub use error::{Error, Result};
pub use factory::{AgentConfig, AgentFactory, AgentFactoryBuilder, QdrantConfig};
pub use rig::providers::openai::responses_api::ResponsesCompletionModel;

/// Type alias for a per-request agent backed by OpenAI's Responses API.
pub type LerpzAgent = rig::agent::Agent<ResponsesCompletionModel>;
