use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::problem::{HandlerResult, Problem, ProblemSchema};
use lerpz_metadata::{MetadataClient, MetadataKind, models::StorageMetadata};
use uuid::Uuid;

use crate::{
    oapi::IMAGES_TAG,
    state::{AppState, DatabasePool, S3Client},
};

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = IMAGES_TAG,
    operation_id = "delete_image",
    summary = "Delete a specific image",
    params(
        ("id" = Uuid, Path, description = "Image ID"),
    ),
    responses(
        (
            status = OK,
            description = "Image deleted successfully"
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
    State(database): State<DatabasePool>,
    State(s3): State<S3Client>,
    Path(id): Path<Uuid>,
) -> HandlerResult<()> {
    let meta_client = lerpz_metadata::Client::from_pool(database);

    tracing::trace!(id = %id, "fetching metadata from database");
    let metadata = meta_client
        .delete(id, MetadataKind::Image)
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
                err.to_string(),
            ),
        })?;

    let (bucket, key) = match metadata {
        lerpz_metadata::Metadata::Image { storage, .. } => match storage {
            StorageMetadata::S3 { bucket, key } => (bucket, key),
            StorageMetadata::ABS { .. } => {
                return Err(Problem::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                    "Unsupported storage backend for image deletion.",
                ));
            }
        },
        _ => {
            return Err(Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Unexpected metadata kind returned for image.",
            ));
        }
    };

    tracing::trace!(bucket = %bucket, key = %key, "deleting image from storage (s3)");
    s3.delete_object()
        .bucket(&bucket)
        .key(&key)
        .send()
        .await
        .map_err(|err| Problem::default().with_error(err))?;

    Ok(())
}
