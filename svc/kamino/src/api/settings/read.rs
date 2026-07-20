use axum::{Json, extract::State};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    api::settings::UserSettings,
    oapi::SETTINGS_TAG,
    state::{AppState, DatabasePool},
};

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "get_settings",
    tag = SETTINGS_TAG,
    summary = "Get account settings",
    description = "Returns the authenticated user's account settings. When the \
        user has never saved any settings, the server-side defaults are \
        returned instead.",
    responses(
        (
            status = OK,
            description = "The user's account settings",
            body = UserSettings
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
    State(database): State<DatabasePool>,
) -> HandlerResult<Json<UserSettings>> {
    let user_id = token.sub;

    let settings = sqlx::query_as!(
        UserSettings,
        r#"SELECT
            theme AS "theme: _",
            notify_product_updates,
            notify_activity_digest,
            notify_security_alerts
        FROM user_settings
        WHERE user_id = $1"#,
        &user_id,
    )
    .fetch_optional(&database)
    .await?;

    Ok(Json(settings.unwrap_or_default()))
}
