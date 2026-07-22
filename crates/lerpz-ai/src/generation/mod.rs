//! Family-aware AI generation.
//!
//! AI providers expose image, video and chat generation in subtly different
//! ways, so generation is dispatched through a [`Family`] — the provider family
//! a model belongs to (e.g. OpenAI, Anthropic, Google). This is distinct from
//! the Portkey *provider* slug used purely for routing: a family decides *how* a
//! request is issued and how its response is interpreted, whereas the provider
//! slug only decides *where* the gateway forwards it.
//!
//! Every family currently shares the [`Family::Default`] behaviour, which is the
//! implementation that works across the providers routed through Portkey today.
//! New families can override individual generation kinds by adding a variant and
//! a match arm, without touching the callers.
//!
//! Each generation kind produces a normalized, boxed event stream
//! ([`ImageStream`], [`VideoStream`], [`ChatStream`]) so that different families
//! can back the same interface with different implementations. Persisting the
//! results and formatting them for transport is left to the caller.

use std::pin::Pin;

use async_openai::{
    Client, config::Config, error::OpenAIError, types::chat::CreateChatCompletionRequest,
};
use tokio_stream::Stream;

use crate::portkey::UpstreamError;

mod chat;
mod image;
mod video;

pub use chat::ChatEvent;
pub use image::{ImageEvent, ImageRequest};
pub use video::{VideoEvent, VideoJob, VideoRequest};

/// A boxed stream of image generation events.
pub type ImageStream = Pin<Box<dyn Stream<Item = Result<ImageEvent, UpstreamError>> + Send>>;
/// A boxed stream of video generation events.
pub type VideoStream = Pin<Box<dyn Stream<Item = Result<VideoEvent, UpstreamError>> + Send>>;
/// A boxed stream of chat completion events.
pub type ChatStream = Pin<Box<dyn Stream<Item = Result<ChatEvent, UpstreamError>> + Send>>;

/// The provider family a model belongs to.
///
/// A family determines how generation requests are issued and how responses are
/// interpreted. Resolve one with [`Family::from_name`] and dispatch generation
/// through the methods below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    /// The default behaviour, used for any family without special handling.
    Default,
}

impl Family {
    /// Resolves a model family name to its [`Family`] handler.
    ///
    /// Unknown or unspecified families fall back to [`Family::Default`].
    #[allow(
        clippy::match_single_binding,
        reason = "kept as a match so bespoke families can be added as arms"
    )]
    pub fn from_name(name: Option<&str>) -> Self {
        match name.map(str::trim) {
            // Every family currently uses the default behaviour. Add arms here
            // as families gain bespoke image/video/chat handling.
            _ => Family::Default,
        }
    }

    /// Starts streaming image generation for `request`.
    ///
    /// The returned stream yields [`ImageEvent`]s; an error here means the
    /// request could not be started at all.
    pub async fn generate_image<C: Config>(
        self,
        client: &Client<C>,
        request: ImageRequest,
    ) -> Result<ImageStream, UpstreamError> {
        match self {
            Family::Default => image::generate(client, request).await,
        }
    }

    /// Starts a video generation job for `request`.
    ///
    /// Video providers render asynchronously, so this returns a [`VideoJob`]
    /// that must be [polled](VideoJob::poll) to completion. An error here means
    /// the job could not be created.
    pub async fn start_video<C: Config>(
        self,
        client: &Client<C>,
        request: VideoRequest,
    ) -> Result<VideoJob, UpstreamError> {
        match self {
            Family::Default => video::start(client, request).await,
        }
    }

    /// Starts streaming a chat completion for `request`.
    ///
    /// The returned stream yields [`ChatEvent`]s. The error type mirrors
    /// `async-openai` so callers can surface start-up failures directly.
    pub async fn chat_stream<C: Config>(
        self,
        client: &Client<C>,
        request: CreateChatCompletionRequest,
    ) -> Result<ChatStream, OpenAIError> {
        match self {
            Family::Default => chat::stream(client, request).await,
        }
    }
}
