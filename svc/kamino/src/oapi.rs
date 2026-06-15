use utoipa::openapi::SecurityRequirement;
use utoipa::openapi::security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::config::CONFIG;

pub(crate) const CHATS_TAG: &str = "chats";
pub(crate) const IMAGES_TAG: &str = "images";
pub(crate) const MODELS_TAG: &str = "models";
pub(crate) const GROUPS_TAG: &str = "groups";
pub(crate) const ORGS_TAG: &str = "orgs";
pub(crate) const AGENTS_TAG: &str = "agents";
pub(crate) const SESSIONS_TAG: &str = "sessions";
pub(crate) const HEALTH_TAG: &str = "health";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lerpz AI",
        description = "Lerpz AI backend API",
        contact(
            name = "Kasper Jønsson",
            email = "kas@lerpz.com",
        ),
    ),
    modifiers(&EntraAuth),
    tags(
        (name = CHATS_TAG, description = "Manage AI conversations and stream responses via Server-Sent Events."),
        (name = IMAGES_TAG, description = "Generate and edit images using AI models with real-time streaming previews."),
        (name = MODELS_TAG, description = "Discover and manage the AI models available to your organization."),
        (name = GROUPS_TAG, description = "Manage user groups and their access to organizational resources."),
        (name = ORGS_TAG, description = "Manage organizations, their members, and top-level settings."),
        (name = AGENTS_TAG, description = "Manage AI agents and their configuration."),
        (name = SESSIONS_TAG, description = "Manage agent sessions and their lifecycle."),
        (name = HEALTH_TAG, description = "Monitor API health and verify connectivity to backing services."),
    )
)]
pub(crate) struct ApiDoc;

/// Injects the Microsoft Entra ID OAuth2 authorization-code security scheme
/// into the generated OpenAPI document and marks every operation as requiring it.
struct EntraAuth;

impl Modify for EntraAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let tenant_id = CONFIG.ENTRA_ID_TENANT_ID.as_ref();
        let client_scope = CONFIG.ENTRA_ID_SCOPE.as_ref();

        let ms_url = "https://login.microsoftonline.com";
        let auth_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/authorize");
        let token_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/token");

        let oauth2_scheme = SecurityScheme::OAuth2(OAuth2::new([Flow::AuthorizationCode(
            AuthorizationCode::new(
                auth_url,
                token_url,
                Scopes::from_iter([(client_scope, "Default API access")]),
            ),
        )]));

        openapi
            .components
            .get_or_insert_with(utoipa::openapi::Components::new)
            .add_security_scheme("oauth2", oauth2_scheme);

        openapi
            .security
            .get_or_insert_with(Vec::new)
            .push(SecurityRequirement::new("oauth2", [client_scope]));
    }
}
