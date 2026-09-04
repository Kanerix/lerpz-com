use axum::{
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

#[utoipa::path(
    method(delete),
    path = "/{id}",
    operation_id = "delete_model",
    tag = MODELS_TAG,
    summary = "Delete a model",
    description = "Permanently deletes a model by its identifier.",
    params(
        ("id" = Uuid, Path, description = "Model ID"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "Model deleted"
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
) -> HandlerResult<StatusCode> {
    let result = sqlx::query!("DELETE FROM models WHERE id = $1", &id)
        .execute(&database)
        .await?;

    if result.rows_affected() == 0 {
        return Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested model was not found.",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
