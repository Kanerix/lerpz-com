use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use uuid::Uuid;

use crate::{
    oapi::MODELS_TAG,
    state::{AppState, DatabasePool},
};

use super::{Model, ModelFamily};

#[utoipa::path(
    method(get),
    path = "/{id}",
    tag = MODELS_TAG,
    operation_id = "get_model",
    summary = "Get a specific model",
    description = "Returns a single model by its identifier.",
    params(
        ("id" = Uuid, Path, description = "Model ID"),
    ),
    responses(
        (
            status = OK,
            description = "The requested model",
            body = Model
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
) -> HandlerResult<Json<Model>> {
    let model = sqlx::query_as!(
        Model,
        r#"SELECT
            id,
            display_name,
            description,
            family AS "family: ModelFamily",
            deployment_name,
            provider,
            modalities,
            settings,
            created_at,
            updated_at
        FROM models
        WHERE id = $1"#,
        &id,
    )
    .fetch_optional(&database)
    .await?;

    match model {
        Some(model) => Ok(Json(model)),
        None => Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested model was not found.",
        )),
    }
}
