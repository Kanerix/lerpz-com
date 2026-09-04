use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    config::CONFIG,
    oapi::IMAGES_TAG,
    state::{AppState, DatabasePool},
};

/// Default page size when the client does not specify one.
const DEFAULT_LIMIT: i64 = 24;
/// Maximum page size a client may request.
const MAX_LIMIT: i64 = 100;

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListQuery {
    /// Return images older than this ID (exclusive).
    ///
    /// Pass the `next_cursor` from the previous page to fetch the following
    /// one; omit for the first page.
    cursor: Option<Uuid>,
    /// Maximum number of images to return. Defaults to 24, capped at 100.
    limit: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImageItem {
    /// Unique image ID.
    id: Uuid,
    /// Publicly accessible URL served directly from the storage bucket, which
    /// acts as a CDN.
    url: String,
    /// Prompt the image was generated from.
    prompt: String,
    /// Model that generated the image.
    model: String,
    /// Optional AI-generated title.
    title: Option<String>,
    /// Optional AI-generated tags.
    tags: Vec<String>,
    /// Container format (e.g. `png`, `jpeg`).
    format: String,
    /// Image width in pixels.
    width: i32,
    /// Image height in pixels.
    height: i32,
    /// When the image was created.
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImageListResponse {
    /// The page of images, newest first.
    items: Vec<ImageItem>,
    /// Cursor to pass as `cursor` for the next page, or `null` when the end has
    /// been reached.
    next_cursor: Option<Uuid>,
}

/// Build a public URL for a stored object.
///
/// The storage bucket is exposed publicly and acts as a lightweight CDN, so a
/// stable URL can be derived directly from the bucket and key. Path-style
/// addressing is used to match the S3 client configuration
/// (`force_path_style`).
fn public_url(bucket: &str, key: &str) -> String {
    format!(
        "{}/{}/{}",
        CONFIG.AWS_S3_ENDPOINT.trim_end_matches('/'),
        bucket,
        key,
    )
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_images",
    tag = IMAGES_TAG,
    summary = "List generated images",
    description = "Returns a page of generated images, newest first, using \
        simple cursor-based pagination. Each item carries a public `url` served \
        from the storage bucket (acting as a CDN). Pass the returned \
        `next_cursor` back as the `cursor` query parameter to load the next \
        page; a `null` cursor means there are no more images.",
    params(ListQuery),
    responses(
        (
            status = OK,
            description = "A page of images",
            body = ImageListResponse,
            content_type = "application/json"
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
    _token: AzureAccessToken,
    State(database): State<DatabasePool>,
    Query(params): Query<ListQuery>,
) -> HandlerResult<Json<ImageListResponse>> {
    let limit = params.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

    // `id` is a uuidv7, so it is monotonic with creation time. Ordering by it
    // descending yields newest-first, and `id < cursor` is a cheap,
    // index-backed way to page. Fetch one extra row to detect whether another
    // page exists without a separate count query.
    tracing::trace!(cursor = ?params.cursor, %limit, "listing images");
    let rows = sqlx::query!(
        r#"SELECT id, prompt, model, title, tags,
                  storage_bucket, storage_key, format, width, height, created_at
           FROM image_metadata
           WHERE $1::uuid IS NULL OR id < $1
           ORDER BY id DESC
           LIMIT $2"#,
        params.cursor,
        limit + 1,
    )
    .fetch_all(&database)
    .await?;

    let has_more = rows.len() as i64 > limit;

    let items: Vec<ImageItem> = rows
        .into_iter()
        .take(limit as usize)
        .map(|r| ImageItem {
            url: public_url(&r.storage_bucket, &r.storage_key),
            id: r.id,
            prompt: r.prompt,
            model: r.model,
            title: r.title,
            tags: r.tags.unwrap_or_default(),
            format: r.format,
            width: r.width,
            height: r.height,
            created_at: r.created_at,
        })
        .collect();

    let next_cursor = if has_more {
        items.last().map(|item| item.id)
    } else {
        None
    };

    Ok(Json(ImageListResponse { items, next_cursor }))
}
