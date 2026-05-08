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
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};

use crate::{
    config::CONFIG,
    oapi::IMAGES_TAG,
    state::{AppState, OpenAI},
};

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

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_image",
    tag = IMAGES_TAG,
    summary = "Create a new image",
    responses(
        (
            status = OK,
            description = "SSE stream of image generation events. Events: \
                partial_image (base64 partial render), \
                completed_image (base64 final image), \
                error (error message)",
            content_type = "text/event-stream"
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid request body",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    State(openai): State<OpenAI>,
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
    let stream = openai.images().generate_stream(request).await?;

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

        dbg!(&event);

        Ok(event)
    });

    Ok(Sse::new(sse_stream))
}
