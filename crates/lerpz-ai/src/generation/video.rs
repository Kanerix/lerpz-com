use std::future::Future;

use async_openai::{Client, config::Config};

use super::{UpstreamError, VideoStream};

/// Routing details for Google's Vertex AI, used for Veo via Portkey.
#[cfg(feature = "portkey")]
#[derive(Debug, Clone)]
pub struct VertexConfig {
    /// Vertex AI regional base URL, sent to Portkey as the
    /// `x-portkey-custom-host`.
    pub custom_host: String,
    /// Google Cloud project id that owns the Vertex deployment.
    pub project_id: String,
    /// Vertex AI location/region.
    pub location: String,
}

/// Parameters for a video generation request.
#[derive(Debug, Clone)]
pub struct VideoRequest {
    /// Prompt describing the desired video.
    pub prompt: String,
    /// Model (deployment name) to route the request to.
    pub model: String,
    /// Desired aspect ratio.
    pub aspect_ratio: Option<String>,
    /// Clip length in seconds. Clamped to the range the provider supports.
    pub duration: Option<u16>,
    /// Vertex AI routing, required by the Veo (Google) path through Portkey.
    #[cfg(feature = "portkey")]
    pub vertex: Option<VertexConfig>,
}

/// An event emitted while a video generation job completes.
#[derive(Debug, Clone)]
pub enum VideoEvent {
    /// The finished video, ready to persist.
    Completed {
        /// The decoded video bytes.
        bytes: Vec<u8>,
        /// Container format (e.g. `mp4`).
        format: String,
        /// Output width in pixels.
        width: u32,
        /// Output height in pixels.
        height: u32,
        /// Clip length in seconds.
        duration: u32,
    },
    /// The provider only exposed a link and the asset could not be downloaded,
    /// so it cannot be persisted but can still be played by the caller.
    Link {
        /// A URL the finished video can be played from.
        url: String,
    },
}

/// A created, long-running video generation operation awaiting completion.
pub struct VideoJob {
    /// The provider's operation identifier, useful for logging.
    pub operation_name: String,
    stream: VideoStream,
}

impl VideoJob {
    /// Creates a job from the provider's operation identifier and poll stream.
    #[cfg(feature = "portkey")]
    pub(crate) fn new(operation_name: String, stream: VideoStream) -> Self {
        Self {
            operation_name,
            stream,
        }
    }

    /// Polls the operation until the video is ready, then yields it.
    pub fn poll(self) -> VideoStream {
        self.stream
    }
}

/// Starts long-running video generation through a provider.
pub trait VideoGeneration {
    /// The configuration used by this provider's `async-openai` client.
    type Config: Config;

    /// Creates a job for `request`. Call [`VideoJob::poll`] to await its result.
    fn generate_video(
        &self,
        client: &Client<Self::Config>,
        request: VideoRequest,
    ) -> impl Future<Output = Result<VideoJob, UpstreamError>> + Send;
}
