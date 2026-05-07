use axum::Json;
use lerpz_axum::problem::{ProblemSchema, HandlerResult};
use serde::Serialize;
use utoipa::ToSchema;

use crate::oapi::MODELS_TAG;

#[derive(Debug, Serialize, ToSchema)]
pub struct Models {
    /// List of available AI models
    models: Vec<Model>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Model {
    /// Human-readable model name (e.g. `gpt-image-1`)
    name: String,
    /// Namespaced slug used when making inference requests (e.g. `@azure/gpt-image-1`)
    slug: String,
    /// Provider family identifier (e.g. `openai`, `google-ai`)
    family: String,
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_models",
    tag = MODELS_TAG,
    summary = "Get available models",
    description = "Returns the list of AI models available to the authenticated user. \
        Each model includes a human-readable name, a namespaced slug used when making \
        inference requests, and a provider family identifier (e.g. `openai`, `google-ai`).",
    responses(
        (
            status = OK,
            description = "List of available models",
            body = Models
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
pub async fn handler() -> HandlerResult<Json<Models>> {
    let models = vec![
        Model {
            name: "gpt-image-1".into(),
            slug: "@azure/gpt-image-1".into(),
            family: "openai".into(),
        },
        Model {
            name: "gemini-2.5-flash-image".into(),
            slug: "@google-ai/gemini-2.5-flash-image".into(),
            family: "google-ai".into(),
        },
    ];

    Ok(Json(Models { models }))
}
