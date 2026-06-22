//! OpenAPI documentation definition for the AI agent HTTP service.

use lerpz_axum::oapi::EntraAuth;
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
    tags(
        (name = AGENT_TAG, description = "AI agent chat endpoints"),
    )
)]
pub(crate) struct ApiDoc;

/// Build the OpenAPI document with the Entra ID security scheme applied.
pub(crate) fn api_doc() -> utoipa::openapi::OpenApi {
    let mut openapi = ApiDoc::openapi();

    EntraAuth::new(
        CONFIG.ENTRA_ID_TENANT_ID.as_ref(),
        CONFIG.ENTRA_ID_SCOPE.as_ref(),
    )
    .modify(&mut openapi);

    openapi
}
