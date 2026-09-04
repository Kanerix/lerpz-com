use std::convert::Infallible;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{Sse, sse::Event},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use image::ImageReader;
use lerpz_ai::generation::{Family, ImageEvent, ImageRequest as ImageGenRequest};
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
    ///
    /// This will default to a predefined model if not provided.
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
                partial_image ({ b64, format } partial render), \
                completed_image ({ b64, format } final image), \
                saved ({ id } persisted metadata), \
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

    // Resolve the model's family so generation can be dispatched to the right
    // provider behaviour. Unknown models fall back to the default family.
    let model_family = sqlx::query_scalar!(
        "SELECT family FROM models WHERE deployment_name = $1 LIMIT 1",
        &model_name,
    )
    .fetch_optional(&database)
    .await?;
    let family = Family::from_name(model_family.as_deref());

    let request = ImageGenRequest {
        prompt: body.prompt.clone(),
        model: model_name.clone(),
        amount: body.amount.unwrap_or(1),
        user: token.upn.clone(),
    };

    let mut stream = family.generate_image(openai.as_ref(), request).await.map_err(|err| {
        Problem::new(
            StatusCode::BAD_GATEWAY,
            "Image generation failed",
            err.message,
        )
    })?;

    let meta_client = lerpz_metadata::Client::from_pool(database);
    let prompt = body.prompt;

    let sse_stream = async_stream::stream! {
        while let Some(event) = stream.next().await {
            let event = match event {
                Ok(event) => event,
                Err(upstream) => {
                    if upstream.is_user() {
                        tracing::warn!(reason = %upstream.message, "image generation rejected by provider");
                    } else {
                        tracing::error!("image generation failed: {}", upstream.message);
                    }
                    yield Ok(Event::default()
                        .event("error")
                        .json_data(&upstream.message)
                        .expect("failed to serialize error event"));
                    break;
                }
            };

            match event {
                ImageEvent::Partial { b64, format } => {
                    yield Ok(Event::default()
                        .event("partial_image")
                        .json_data(json!({ "b64": b64, "format": format }))
                        .expect("failed to serialize partial_image event"));
                }
                ImageEvent::Completed { b64, format } => {
                    yield Ok(Event::default()
                        .event("completed_image")
                        .json_data(json!({ "b64": b64, "format": format }))
                        .expect("failed to serialize completed_image event"));

                    tracing::trace!("decoding complete image");
                    let image_bytes: Vec<u8> = match BASE64.decode(&b64) {
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
                                tracing::error!("failed to read image: {err}");
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
                            prompt: prompt.clone(),
                            model: model_name.clone(),
                        },
                        storage: StorageMetadata::S3 {
                            bucket: CONFIG.AWS_S3_BUCKET.to_string(),
                            key,
                        },
                        analysis: None,
                        format,
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
            }
        }
    };

    Ok(Sse::new(sse_stream))
}
