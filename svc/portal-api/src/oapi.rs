use utoipa::openapi::SecurityRequirement;
use utoipa::openapi::security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::config::CONFIG;

pub(crate) const CHATS_TAG: &str = "chats";
pub(crate) const IMAGES_TAG: &str = "images";
pub(crate) const MODELS_TAG: &str = "models";
pub(crate) const GROUPS_TAG: &str = "groups";
pub(crate) const ORGS_TAG: &str = "orgs";
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
        (name = CHATS_TAG, description = "Chat API endpoints"),
        (name = IMAGES_TAG, description = "Image API endpoints"),
        (name = MODELS_TAG, description = "Models API endpoints"),
        (name = GROUPS_TAG, description = "Groups API endpoints"),
        (name = ORGS_TAG, description = "Orgs API endpoints"),
        (name = HEALTH_TAG, description = "Health API endpoints"),
    )
)]
pub(crate) struct ApiDoc;

struct EntraAuth;

impl Modify for EntraAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let tenant_id = &CONFIG.ENTRA_ID_TENANT_ID;
        let client_id = &CONFIG.ENTRA_ID_CLIENT_ID;
        let client_scope = &CONFIG.ENTRA_ID_CLIENT_SCOPE;

        let ms_url = "https://login.microsoftonline.com";
        let auth_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/authorize");
        let token_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/token");

        let oauth2_scheme = SecurityScheme::OAuth2(OAuth2::new([Flow::AuthorizationCode(
            AuthorizationCode::new(
                auth_url,
                token_url,
                Scopes::from_iter([(
                    String::from(client_scope),
                    String::from("Default API access"),
                )]),
            ),
        )]));

        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme("oauth2", oauth2_scheme);
        } else {
            let mut components = utoipa::openapi::Components::new();
            components.add_security_scheme("oauth2", oauth2_scheme);
            openapi.components = Some(components);
        }

        openapi
            .security
            .get_or_insert_with(Vec::new)
            .push(SecurityRequirement::new("oauth2", [client_scope]));
    }
}
