//! Default image generation.
//!
//! Streams image generation through the OpenAI-compatible images API, emitting
//! partial renders followed by the completed image. Decoding, measuring and
//! persisting the result is the caller's responsibility.

use async_openai::{
    Client,
    config::Config,
    types::images::{
        CreateImageRequestArgs, ImageGenCompletedEvent, ImageGenPartialImageEvent,
        ImageGenStreamEvent, ImageModel, ImageQuality, ImageSize,
    },
};
use tokio_stream::StreamExt as _;

use crate::{
    generation::ImageStream,
    portkey::{UpstreamError, classify_error},
};

/// Number of partial renders to request while an image is being generated.
const PARTIAL_IMAGES: u8 = 3;

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

/// Starts streaming image generation.
pub(super) async fn generate<C: Config>(
    client: &Client<C>,
    request: ImageRequest,
) -> Result<ImageStream, UpstreamError> {
    let mut builder = CreateImageRequestArgs::default();
    builder
        .model(ImageModel::Other(request.model))
        .prompt(request.prompt)
        .n(request.amount)
        .quality(ImageQuality::Low)
        .size(ImageSize::S1024x1024)
        .partial_images(PARTIAL_IMAGES)
        .stream(true);

    if let Some(user) = request.user {
        builder.user(user);
    }

    let req = builder
        .build()
        .map_err(|err| UpstreamError::provider(err.to_string()))?;

    // Portkey passes upstream provider errors straight through, often in a
    // non-OpenAI shape `async-openai` can't deserialize. Surface the classified
    // upstream error instead of letting it collapse into something opaque.
    let mut stream = client
        .images()
        .generate_stream(req)
        .await
        .map_err(|err| classify_error(&err.to_string()))?;

    Ok(Box::pin(async_stream::stream! {
        tracing::trace!("starting image stream");
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(ImageGenStreamEvent::PartialImage(ImageGenPartialImageEvent {
                    b64_json,
                    partial_image_index,
                    output_format,
                    ..
                })) => {
                    tracing::trace!(index = %partial_image_index, format = %output_format, "generated partial image");
                    yield Ok(ImageEvent::Partial {
                        b64: b64_json,
                        format: output_format.to_string(),
                    });
                }
                Ok(ImageGenStreamEvent::Completed(ImageGenCompletedEvent {
                    b64_json,
                    output_format,
                    ..
                })) => {
                    tracing::trace!(format = %output_format, "generated complete image");
                    yield Ok(ImageEvent::Completed {
                        b64: b64_json,
                        format: output_format.to_string(),
                    });
                }
                Err(err) => {
                    yield Err(classify_error(&err.to_string()));
                    break;
                }
            }
        }
    }))
}
