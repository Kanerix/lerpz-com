use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use lerpz_metadata::{
    Metadata, MetadataClient, MetadataKind, models::StorageMetadata,
};
use uuid::Uuid;

use crate::{
    oapi::IMAGES_TAG,
    state::{AppState, DatabasePool, OpenAI, S3Client},
};

use super::ImageAnalysisResponse;

#[utoipa::path(
    method(post),
    path = "/{id}/analysis",
    operation_id = "analyze_image",
    tag = IMAGES_TAG,
    summary = "Analyse an image",
    description = "Runs a vision model over a previously generated image to \
        produce a descriptive title and a set of tags, persists them to the \
        image's metadata, and returns them. Re-running the analysis overwrites \
        any existing title and tags.",
    params(
        ("id" = Uuid, Path, description = "Image ID"),
    ),
    responses(
        (
            status = OK,
            description = "The generated analysis",
            body = ImageAnalysisResponse,
            content_type = "application/json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Image not found",
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
    Path(id): Path<Uuid>,
) -> HandlerResult<Json<ImageAnalysisResponse>> {
    let meta_client = lerpz_metadata::Client::from_pool(database);

    tracing::trace!(%id, "fetching image metadata from database");
    let metadata = meta_client
        .get(id, MetadataKind::Image)
        .await
        .map_err(|err| match err {
            lerpz_metadata::error::Error::NotFound(_) => Problem::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                "The requested image was not found.",
            ),
            err => Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Failed to fetch image metadata.",
            )
            .with_error(err),
        })?;

    let Metadata::Image {
        general,
        generation,
        storage,
        format,
        width,
        height,
        ..
    } = metadata
    else {
        tracing::error!(%id, "unexpected metadata kind returned for image");
        return Err(Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "Unexpected metadata kind returned for image.",
        ));
    };

    // The image bucket is private, so the model provider cannot fetch it by
    // URL. Download the bytes ourselves (we hold the credentials) and inline
    // them as a base64 data URL.
    let (bucket, key) = match &storage {
        StorageMetadata::S3 { bucket, key } => (bucket, key),
        StorageMetadata::ABS { .. } => {
            tracing::error!(%id, "unsupported storage backend for image analysis");
            return Err(Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Unsupported storage backend for image analysis.",
            ));
        }
    };

    tracing::trace!(%id, %bucket, %key, "fetching image bytes from storage");
    let object = s3
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|err| {
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Failed to fetch image from storage.",
            )
            .with_error(err)
        })?;

    let bytes = object.body.collect().await.map_err(|err| {
        Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "Failed to read image from storage.",
        )
        .with_error(err)
    })?;

    tracing::trace!(%id, "requesting image analysis from model");
    let analysis = super::analyze_bytes(&openai, &format, &bytes.into_bytes(), token.upn).await?;

    let response = ImageAnalysisResponse {
        title: analysis.title.clone(),
        tags: analysis.tags.clone(),
    };

    tracing::trace!(%id, "persisting image analysis to database");
    meta_client
        .update(
            id,
            Metadata::Image {
                general,
                generation,
                storage,
                analysis: Some(analysis),
                format,
                width,
                height,
            },
        )
        .await
        .map_err(|err| {
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Failed to persist image analysis.",
            )
            .with_error(err)
        })?;

    Ok(Json(response))
}
