use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    oapi::MODELS_TAG,
    state::{AppState, DatabasePool},
};

use super::{Model, ModelSettings};

/// Parameters for updating an existing model.
///
/// Every field is optional; omitted fields are left unchanged.
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateModelRequest {
    /// Human-readable name shown in UIs.
    #[serde(default)]
    display_name: Option<String>,
    /// Longer description of the model.
    #[serde(default)]
    description: Option<String>,
    /// Provider family the model belongs to.
    #[serde(default)]
    family: Option<String>,
    /// Portkey deployment name used when routing requests.
    #[serde(default)]
    deployment_name: Option<String>,
    /// Portkey provider slug the deployment lives under.
    #[serde(default)]
    provider: Option<String>,
    /// Input/output modalities the model supports (e.g. `text`, `image`).
    #[serde(default)]
    modalities: Option<Vec<String>>,
    /// Provider/runtime settings as a JSON object.
    #[serde(default)]
    #[schema(value_type = Option<ModelSettings>)]
    settings: Option<serde_json::Value>,
}

#[utoipa::path(
    method(patch),
    path = "/{id}",
    operation_id = "update_model",
    tag = MODELS_TAG,
    summary = "Update a specific model",
    description = "Partially updates a model. Only the provided fields are changed.",
    params(
        ("id" = Uuid, Path, description = "Model ID"),
    ),
    request_body(
        content = UpdateModelRequest,
        description = "Fields to update",
        content_type = "application/json",
    ),
    responses(
        (
            status = OK,
            description = "The updated model",
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
            status = NOT_FOUND,
            description = "Resource not found",
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
    Path(id): Path<Uuid>,
    State(database): State<DatabasePool>,
    Json(body): Json<UpdateModelRequest>,
) -> HandlerResult<Json<Model>> {
    let model = sqlx::query_as!(
        Model,
        r#"UPDATE models SET
            display_name = COALESCE($2, display_name),
            description = COALESCE($3, description),
            family = COALESCE($4, family),
            deployment_name = COALESCE($5, deployment_name),
            provider = COALESCE($6, provider),
            modalities = COALESCE($7, modalities),
            settings = COALESCE($8, settings)
        WHERE id = $1
        RETURNING
            id,
            display_name,
            description,
            family,
            deployment_name,
            provider,
            modalities,
            settings,
            created_at,
            updated_at"#,
        &id,
        body.display_name.as_deref(),
        body.description.as_deref(),
        body.family.as_deref(),
        body.deployment_name.as_deref(),
        body.provider.as_deref(),
        body.modalities.as_deref(),
        body.settings,
    )
    .fetch_optional(&database)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db) if db.is_unique_violation() => Problem::new(
            StatusCode::CONFLICT,
            "Conflict",
            "A model with the same provider and deployment name already exists.",
        ),
        other => other.into(),
    })?;

    match model {
        Some(model) => Ok(Json(model)),
        None => Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested model was not found.",
        )),
    }
}
