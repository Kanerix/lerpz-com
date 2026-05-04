//! OpenAPI documentation definition for the AI agent HTTP service.

use utoipa::openapi::SecurityRequirement;
use utoipa::openapi::security::{AuthorizationCode, Flow, OAuth2, Scopes, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::config::CONFIG;

pub(crate) const AGENT_TAG: &str = "agent";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lerpz AI Agent",
        description = "Lerpz AI Agent HTTP API",
        contact(
            name = "Kasper Jønsson",
            email = "kas@lerpz.com",
        ),
    ),
    modifiers(&EntraAuth),
    tags(
        (name = AGENT_TAG, description = "AI agent chat endpoints"),
    )
)]
pub(crate) struct ApiDoc;

struct EntraAuth;

impl Modify for EntraAuth {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let tenant_id = &CONFIG.ENTRA_ID_TENANT_ID;
        let client_id = &CONFIG.ENTRA_ID_CLIENT_ID;
        let client_scope = &CONFIG.ENTRA_ID_SCOPE;

        let ms_url = "https://login.microsoftonline.com";
        let auth_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/authorize");
        let token_url = format!("{ms_url}/{tenant_id}/oauth2/v2.0/token");

        let oauth2_scheme = SecurityScheme::OAuth2(OAuth2::new([Flow::AuthorizationCode(
            AuthorizationCode::new(
                auth_url,
                token_url,
                Scopes::from_iter([(client_scope.as_str(), "Default API access")]),
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
