use axum::{Json, extract::State};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    api::settings::{ThemePref, UserSettings},
    oapi::SETTINGS_TAG,
    state::{AppState, DatabasePool},
};

/// Partial update for a user's account settings.
///
/// Every field is optional; omitted fields keep their current value (or fall
/// back to the default when the user has no stored settings yet).
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSettingsRequest {
    /// Preferred colour theme.
    #[serde(default)]
    theme: Option<ThemePref>,
    /// Receive product-update notifications.
    #[serde(default)]
    notify_product_updates: Option<bool>,
    /// Receive the weekly activity digest.
    #[serde(default)]
    notify_activity_digest: Option<bool>,
    /// Receive security alerts.
    #[serde(default)]
    notify_security_alerts: Option<bool>,
}

#[utoipa::path(
    method(patch),
    path = "/",
    operation_id = "update_settings",
    tag = SETTINGS_TAG,
    summary = "Update account settings",
    description = "Updates the authenticated user's account settings, creating \
        them on first use. Only the fields present in the request body are \
        changed; omitted fields keep their current value. Returns the full, \
        updated settings.",
    request_body(
        content = UpdateSettingsRequest,
        description = "The settings fields to change",
        content_type = "application/json",
    ),
    responses(
        (
            status = OK,
            description = "The updated account settings",
            body = UserSettings
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
    Json(body): Json<UpdateSettingsRequest>,
) -> HandlerResult<Json<UserSettings>> {
    let user_id = token.sub;

    let settings = sqlx::query_as!(
        UserSettings,
        r#"INSERT INTO user_settings (
            user_id,
            theme,
            notify_product_updates,
            notify_activity_digest,
            notify_security_alerts
        )
        VALUES (
            $1,
            COALESCE($2::theme_pref, 'system'),
            COALESCE($3, TRUE),
            COALESCE($4, FALSE),
            COALESCE($5, TRUE)
        )
        ON CONFLICT (user_id) DO UPDATE SET
            theme = COALESCE($2::theme_pref, user_settings.theme),
            notify_product_updates = COALESCE($3, user_settings.notify_product_updates),
            notify_activity_digest = COALESCE($4, user_settings.notify_activity_digest),
            notify_security_alerts = COALESCE($5, user_settings.notify_security_alerts)
        RETURNING
            theme AS "theme: _",
            notify_product_updates,
            notify_activity_digest,
            notify_security_alerts"#,
        &user_id,
        body.theme as Option<ThemePref>,
        body.notify_product_updates,
        body.notify_activity_digest,
        body.notify_security_alerts,
    )
    .fetch_one(&database)
    .await?;

    Ok(Json(settings))
}
