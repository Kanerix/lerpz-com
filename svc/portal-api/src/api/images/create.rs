use std::convert::Infallible;

use async_openai::types::images::{
    CreateImageRequestArgs, ImageGenCompletedEvent, ImageGenPartialImageEvent, ImageGenStreamEvent,
    ImageModel, ImageQuality, ImageSize,
};
use axum::{
    Json,
    extract::State,
    response::{Sse, sse::Event},
};
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};

use crate::{config::CONFIG, oapi::IMAGES_TAG, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ImageRequest {
    /// What model to use.
    model: Option<String>,
    /// Prompt that is sent to the model.
    prompt: String,
    /// The amount of images to generate.
    ///
    /// This will default to 1 image if not provided.
    amount: Option<u8>,
}

// #[derive(Debug, Serialize)]
// pub struct PartialImage {
//     image_index: usize,
//     image_data: String,
// }

// #[derive(Debug, Serialize)]
// pub struct CompletedImage {
//     index: usize,
//     data: String,
// }

#[utoipa::path(
    method(post),
    path = "/",
    tag = IMAGES_TAG,
    summary = "Create a new image",
)]
#[axum::debug_handler]
pub async fn handler(
    token: AzureAccessToken,
    State(state): State<AppState>,
    Json(body): Json<ImageRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let model = ImageModel::Other(
        body.model
            .as_deref()
            .unwrap_or(&CONFIG.DEFAULT_IMAGE_MODEL)
            .to_string(),
    );

    let mut request_builder = CreateImageRequestArgs::default();

    request_builder
        .model(model)
        .prompt(body.prompt)
        .n(body.amount.unwrap_or(1))
        .quality(ImageQuality::Low)
        .size(ImageSize::S1024x1024)
        .partial_images(3)
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    };

    let request = request_builder.build()?;

    let client = state.openai;
    let stream = client.images().generate_stream(request).await?;

    let sse_stream = stream.map(|chunk_result| {
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(err) => {
                return Ok(Event::default()
                    .event("error")
                    .json_data(err.to_string())
                    .unwrap());
            }
        };

        let event = match chunk {
            ImageGenStreamEvent::PartialImage(ImageGenPartialImageEvent { b64_json, .. }) => {
                Event::default()
                    .event("partial_image")
                    .json_data(b64_json)
                    .unwrap()
            }
            ImageGenStreamEvent::Completed(ImageGenCompletedEvent { b64_json, .. }) => {
                Event::default()
                    .event("completed_image")
                    .json_data(b64_json)
                    .unwrap()
            }
        };

        Ok(event)
    });

    Ok(Sse::new(sse_stream))
}
