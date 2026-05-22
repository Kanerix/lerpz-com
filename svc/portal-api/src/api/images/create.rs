use std::convert::Infallible;

use async_openai::types::images::{
    CreateImageRequestArgs, ImageGenCompletedEvent, ImageGenPartialImageEvent, ImageGenStreamEvent,
    ImageModel, ImageQuality, ImageSize,
};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{Sse, sse::Event},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use image::ImageReader;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use lerpz_metadata::{
    Metadata, MetadataClient,
    models::{GeneralMetadata, GenerationMetadata, StorageMetadata},
};
use serde::Deserialize;
use serde_json::json;
use tokio_stream::{Stream, StreamExt as _};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    oapi::IMAGES_TAG,
    state::{AppState, DatabasePool, OpenAI, S3Client},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ImageRequest {
    /// Prompt that is sent to the model.
    prompt: String,
    /// What model to use.
    model: Option<String>,
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
    request_body(
        content = ImageRequest,
        description = "Image generation parameters",
        content_type = "application/json",
        example = json!({
            "prompt": "A toller dog!"
        })
    ),
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
    State(database): State<DatabasePool>,
    State(s3): State<S3Client>,
    Json(body): Json<ImageRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let oid = token.oid.ok_or(Problem::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unkown token format",
        "Missing Object ID from token",
    ))?;

    let model_name = body
        .model
        .as_deref()
        .unwrap_or(&CONFIG.DEFAULT_IMAGE_MODEL)
        .to_string();

    let mut request_builder = CreateImageRequestArgs::default();

    request_builder
        .model(ImageModel::Other(model_name.clone()))
        .prompt(body.prompt.clone())
        .n(body.amount.unwrap_or(1))
        .quality(ImageQuality::Low)
        .size(ImageSize::S1024x1024)
        .partial_images(3)
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    };

    let request = request_builder.build()?;
    let mut stream = openai.images().generate_stream(request).await?;
    let meta_client = lerpz_metadata::Client::from_pool(database);

    let sse_stream = async_stream::stream! {
        tracing::trace!("starting image stream");
        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(err) => {
                    yield Ok(Event::default()
                        .event("error")
                        .json_data(err.to_string())
                        .expect("failed to serialize error event"));
                    continue;
                }
            };

            match chunk {
                ImageGenStreamEvent::PartialImage(ImageGenPartialImageEvent { b64_json, partial_image_index,.. }) => {
                    tracing::trace!(index = %partial_image_index, "generated partial image");
                    yield Ok(Event::default()
                        .event("partial_image")
                        .json_data(b64_json)
                        .expect("failed to serialize partial_image event"));
                }
                ImageGenStreamEvent::Completed(ImageGenCompletedEvent { b64_json, output_format, .. }) => {
                    tracing::trace!(format = %output_format, "generated complete image");
                    yield Ok(Event::default()
                        .event("completed_image")
                        .json_data(&b64_json)
                        .expect("failed to serialize completed_image event"));

                    tracing::trace!("decoding complete image");
                    let image_bytes: Vec<u8> = match BASE64.decode(&b64_json) {
                        Ok(bytes) => bytes,
                        Err(err) => {
                            tracing::error!("{err}");
                            yield Ok(Event::default()
                                .event("error")
                                .json_data(err.to_string())
                                .expect("failed to serialize error event"));
                            continue;
                        }
                    };

                    tracing::trace!("reading dimensions of complete image");
                    let (width, height) = {
                        let cursor = std::io::Cursor::new(&image_bytes);
                        match ImageReader::new(cursor).with_guessed_format() {
                            Ok(reader) => match reader.into_dimensions() {
                                Ok(dims) => dims,
                                Err(err) => {
                                    tracing::error!("{err}");
                                    yield Ok(Event::default()
                                        .event("error")
                                        .json_data(err.to_string())
                                        .expect("failed to serialize error event"));
                                    continue;
                                }
                            },
                            Err(err) => {
                                tracing::error!("{err}");
                                yield Ok(Event::default()
                                    .event("error")
                                    .json_data(err.to_string())
                                    .expect("failed to serialize error event"));
                                continue;
                            }
                        }
                    };

                    let id = Uuid::now_v7();
                    let key = format!("images/{oid}/{id}.jpg");
                    let now = Utc::now();

                    tracing::trace!(
                        bucket = %CONFIG.AWS_S3_BUCKET,
                        key = %key,
                        "saving image to storage (s3)",
                    );

                    let metadata = Metadata::Image {
                        general: GeneralMetadata { id, created_at: now, updated_at: now },
                        generation: GenerationMetadata {
                            prompt: body.prompt.clone(),
                            model: model_name.clone(),
                        },
                        storage: StorageMetadata::S3 {
                            bucket: CONFIG.AWS_S3_BUCKET.to_string(),
                            key,
                        },
                        analysis: None,
                        format: output_format.to_string(),
                        width,
                        height,
                    };

                    if let Err(err) = lerpz_metadata::save_to_s3(&s3, &metadata, &image_bytes).await {
                        tracing::error!("{err}");
                        yield Ok(Event::default()
                            .event("error")
                            .json_data(err.to_string())
                            .expect("failed to serialize error event"));
                        continue;
                    }

                    tracing::trace!("persisting metadata to database");
                    match meta_client.insert(metadata).await {
                        Ok(_) => {
                            yield Ok(Event::default()
                                .event("saved")
                                .json_data(json!({ "id": id }))
                                .expect("failed to serialize saved event"));
                        }
                        Err(err) => {
                            tracing::error!("{err}");
                            yield Ok(Event::default()
                                .event("error")
                                .json_data(err.to_string())
                                .expect("failed to serialize error event"));
                        }
                    }
                }
            };
        }
    };

    Ok(Sse::new(sse_stream))
}
