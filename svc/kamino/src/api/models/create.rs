use axum::{Json, extract::State, http::StatusCode};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;

use crate::{
    oapi::MODELS_TAG,
    state::{AppState, DatabasePool},
};

use super::{Model, ModelFamily, ModelSettings};

/// Parameters for creating a new model.
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateModelRequest {
    /// Human-readable name shown in UIs (e.g. `GPT-4o`).
    display_name: String,
    /// Optional longer description of the model.
    #[serde(default)]
    description: Option<String>,
    /// Provider family the model belongs to.
    family: ModelFamily,
    /// Portkey deployment name used when routing requests.
    deployment_name: String,
    /// Portkey provider slug the deployment lives under.
    provider: String,
    /// Provider/runtime settings as a JSON object. Defaults to `{}`.
    #[serde(default)]
    #[schema(value_type = Option<ModelSettings>)]
    settings: Option<serde_json::Value>,
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_model",
    tag = MODELS_TAG,
    summary = "Create a new model",
    description = "Registers a new model and its Portkey routing configuration.",
    request_body(
        content = CreateModelRequest,
        description = "Model creation parameters",
        content_type = "application/json",
    ),
    responses(
        (
            status = CREATED,
            description = "The newly created model",
            body = Model
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
            status = CONFLICT,
            description = "A model with the same provider and deployment name already exists",
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
    Json(body): Json<CreateModelRequest>,
) -> HandlerResult<(StatusCode, Json<Model>)> {
    let settings = body.settings.unwrap_or_else(|| json!({}));

    let model = sqlx::query_as!(
        Model,
        r#"INSERT INTO models
            (display_name, description, family, deployment_name, provider, settings)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            id,
            display_name,
            description,
            family AS "family: ModelFamily",
            deployment_name,
            provider,
            settings,
            created_at,
            updated_at"#,
        &body.display_name,
        body.description.as_deref(),
        body.family as _,
        &body.deployment_name,
        &body.provider,
        settings,
    )
    .fetch_one(&database)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db) if db.is_unique_violation() => Problem::new(
            StatusCode::CONFLICT,
            "Conflict",
            "A model with the same provider and deployment name already exists.",
        ),
        other => other.into(),
    })?;

    Ok((StatusCode::CREATED, Json(model)))
}
