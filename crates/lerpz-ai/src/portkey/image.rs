//! Default image generation.
//!
//! Streams image generation through the OpenAI-compatible images API, emitting
//! partial renders followed by the completed image. Decoding, measuring and
//! persisting the result is the caller's responsibility.

use async_openai::{
    Client,
    types::images::{
        CreateImageRequest, CreateImageRequestArgs, ImageGenCompletedEvent,
        ImageGenPartialImageEvent, ImageGenStream, ImageGenStreamEvent, ImageModel, ImageQuality,
        ImageSize,
    },
};
use tokio_stream::StreamExt as _;

use crate::{
    generation::{
        Family, ImageEvent, ImageGeneration, ImageRequest, ImageStream, UpstreamError,
        classify_error,
    },
    portkey::PortkeyConfig,
};

/// Number of partial renders to request while an image is being generated.
const PARTIAL_IMAGES: u8 = 3;

impl ImageGeneration for Family {
    type Config = PortkeyConfig;

    async fn generate_image(
        &self,
        client: &Client<Self::Config>,
        request: ImageRequest,
    ) -> Result<ImageStream, UpstreamError> {
        let request = create_request(request)?;
        let stream = client
            .images()
            .generate_stream(request)
            .await
            .map_err(|err| classify_error(&err.to_string()))?;

        Ok(image_events(stream))
    }
}

fn create_request(request: ImageRequest) -> Result<CreateImageRequest, UpstreamError> {
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

    builder
        .build()
        .map_err(|err| UpstreamError::provider(err.to_string()))
}

fn image_events(mut stream: ImageGenStream) -> ImageStream {
    Box::pin(async_stream::stream! {
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(ImageGenStreamEvent::PartialImage(ImageGenPartialImageEvent {
                    b64_json,
                    output_format,
                    ..
                })) => {
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
    })
}
