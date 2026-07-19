use axum::{Json, extract::State, http::StatusCode};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    oapi::IMAGES_TAG,
    state::{AppState, OpenAI},
};

use super::ImageAnalysisResponse;

/// Body for analysing a caller-supplied ("bring your own") image.
#[derive(Debug, Deserialize, ToSchema)]
pub struct AnalyzeUploadRequest {
    /// The image to analyse, base64-encoded.
    ///
    /// Both raw base64 and a full `data:` URL (e.g.
    /// `data:image/png;base64,...`) are accepted; the leading data-URL prefix
    /// is stripped before decoding.
    #[schema(content_encoding = "base64")]
    image: String,
}

#[utoipa::path(
    method(post),
    path = "/analysis",
    operation_id = "analyze_uploaded_image",
    tag = IMAGES_TAG,
    summary = "Analyse an uploaded image",
    description = "Runs a vision model over a caller-supplied image to produce a \
        descriptive title and a set of tags, and returns them. Unlike analysing \
        a stored image, the upload is not persisted and nothing is written to \
        the database.",
    request_body(
        content = AnalyzeUploadRequest,
        description = "Base64-encoded image to analyse",
        content_type = "application/json",
    ),
    responses(
        (
            status = OK,
            description = "The generated analysis",
            body = ImageAnalysisResponse,
            content_type = "application/json"
        ),
        (
            status = BAD_REQUEST,
            description = "The uploaded image is missing, not valid base64, or not a recognised image",
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
    Json(body): Json<AnalyzeUploadRequest>,
) -> HandlerResult<Json<ImageAnalysisResponse>> {
    let raw = body.image.trim();

    // Accept a full data URL as well as bare base64 by stripping an optional
    // `data:*;base64,` prefix (everything up to and including the last comma).
    let encoded = raw.rsplit_once(',').map_or(raw, |(_, data)| data);

    let bytes = BASE64.decode(encoded).map_err(|err| {
        Problem::new(
            StatusCode::BAD_REQUEST,
            "Invalid image",
            "The image could not be decoded as base64.",
        )
        .with_error(err)
    })?;

    // Detect the format from the magic bytes rather than trusting the caller,
    // and derive the MIME subtype (e.g. `png`, `jpeg`) for the data URL.
    let format = image::guess_format(&bytes).map_err(|err| {
        Problem::new(
            StatusCode::BAD_REQUEST,
            "Invalid image",
            "The uploaded file is not a recognised image.",
        )
        .with_error(err)
    })?;
    let subtype = format.to_mime_type().strip_prefix("image/").unwrap_or("png");

    tracing::trace!("requesting analysis for uploaded image");
    let analysis = super::analyze_bytes(&openai, subtype, &bytes, token.upn).await?;

    Ok(Json(ImageAnalysisResponse {
        title: analysis.title,
        tags: analysis.tags,
    }))
}
