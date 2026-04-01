//! OpenAPI documentation definition for the AI agent HTTP service.

use utoipa::OpenApi;

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