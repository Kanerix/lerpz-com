use std::future::Future;

use async_openai::{Client, config::Config};

use super::{ImageStream, UpstreamError};

/// Parameters for an image generation request.
#[derive(Debug, Clone)]
pub struct ImageRequest {
    /// Prompt describing the desired image.
    pub prompt: String,
    /// Model (deployment name) to route the request to.
    pub model: String,
    /// Number of images to generate.
    pub amount: u8,
    /// Optional stable end-user identifier forwarded for abuse monitoring.
    pub user: Option<String>,
}

/// An event emitted while streaming image generation.
#[derive(Debug, Clone)]
pub enum ImageEvent {
    /// A partial, in-progress render.
    Partial {
        /// Base64-encoded image data.
        b64: String,
        /// Image format (e.g. `png`, `jpeg`).
        format: String,
    },
    /// The final rendered image.
    Completed {
        /// Base64-encoded image data.
        b64: String,
        /// Image format (e.g. `png`, `jpeg`).
        format: String,
    },
}

/// Streams image generation through a provider.
pub trait ImageGeneration {
    /// The configuration used by this provider's `async-openai` client.
    type Config: Config;

    /// Starts streaming image generation for `request`.
    ///
    /// An error here means the request could not be started at all.
    fn generate_image(
        &self,
        client: &Client<Self::Config>,
        request: ImageRequest,
    ) -> impl Future<Output = Result<ImageStream, UpstreamError>> + Send;
}
