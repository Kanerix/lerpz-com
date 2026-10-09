//! Shared generation requests, events and provider traits.
//!
//! Each provider implements [`ChatGeneration`], [`ImageGeneration`] and
//! [`VideoGeneration`] to turn requests into event streams. Persisting results
//! and formatting them for transport is left to the caller.

use std::pin::Pin;

use tokio_stream::Stream;

mod chat;
mod error;
mod image;
mod video;

pub use chat::{ChatEvent, ChatGeneration, ChatMessage, ChatRequest};
pub use error::{ErrorKind, UpstreamError, classify_error, humanize_error};
pub use image::{ImageEvent, ImageGeneration, ImageRequest};
#[cfg(feature = "portkey")]
pub use video::VertexConfig;
pub use video::{VideoEvent, VideoGeneration, VideoJob, VideoRequest};

/// A boxed stream of image generation events.
pub type ImageStream = Pin<Box<dyn Stream<Item = Result<ImageEvent, UpstreamError>> + Send>>;
/// A boxed stream of video generation events.
pub type VideoStream = Pin<Box<dyn Stream<Item = Result<VideoEvent, UpstreamError>> + Send>>;
/// A boxed stream of chat completion events.
pub type ChatStream = Pin<Box<dyn Stream<Item = Result<ChatEvent, UpstreamError>> + Send>>;

/// The model family used by Portkey to select provider-specific behaviour.
#[cfg(feature = "portkey")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// The default behaviour, used for any family without special handling.
    Default,
    /// Google (Gemini/Veo). Video uses Veo's native long-running endpoint.
    Google,
}
