use crate::state::AppState;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

mod read;
mod update;

/// A user's preferred colour theme.
///
/// Maps to the `theme_pref` enum type defined in the database schema. `system`
/// defers to the user's operating-system preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "theme_pref", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ThemePref {
    Light,
    Dark,
    System,
}

impl Default for ThemePref {
    fn default() -> Self {
        Self::System
    }
}

/// The authenticated user's account settings.
///
/// Backs the settings surface in the dashboard: the appearance (theme) and the
/// notification preferences. Every field is always present; when a user has no
/// stored settings the server responds with the defaults.
#[derive(Debug, Serialize, ToSchema)]
pub struct UserSettings {
    /// Preferred colour theme.
    pub theme: ThemePref,
    /// Receive product-update notifications (new tools, models and features).
    pub notify_product_updates: bool,
    /// Receive the weekly activity digest.
    pub notify_activity_digest: bool,
    /// Receive security alerts (new-device sign-ins, role changes).
    pub notify_security_alerts: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: ThemePref::default(),
            notify_product_updates: true,
            notify_activity_digest: false,
            notify_security_alerts: true,
        }
    }
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(read::handler))
        .routes(routes!(update::handler))
}
