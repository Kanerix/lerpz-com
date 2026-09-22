use axum::{Json, extract::State};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    oapi::MODELS_TAG,
    state::{AppState, DatabasePool},
};

use super::ModelResponse;

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_models",
    tag = MODELS_TAG,
    summary = "Get available models",
    description = "Returns the list of AI models available to the authenticated user, \
        ordered by display name.",
    responses(
        (
            status = OK,
            description = "List of available models",
            body = Vec<ModelResponse>
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
) -> HandlerResult<Json<Vec<ModelResponse>>> {
    let models = sqlx::query_as!(
        ModelResponse,
        r#"SELECT
            id,
            display_name,
            description,
            family,
            deployment_name,
            provider,
            modalities,
            settings,
            created_at,
            updated_at
        FROM models
        ORDER BY display_name ASC"#,
    )
    .fetch_all(&database)
    .await?;

    Ok(Json(models))
}
