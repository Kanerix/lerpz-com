use lerpz_axum::oapi::EntraAuth;
use utoipa::{Modify, OpenApi};

use crate::config::CONFIG;

pub(crate) const VOLUMES_TAG: &str = "volumes";
pub(crate) const RUNTIMES_TAG: &str = "runtimes";
pub(crate) const HEALTH_TAG: &str = "health";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lerpz Forge — API references",
        description = "The internal provisioning API for agent infrastructure. \
            Forge turns a request for agent capacity into the Kubernetes objects \
            that back it: persistent memory volumes and the container runtimes \
            that mount them. It is not exposed publicly — `artoo` and `api` are \
            its only callers.",
        contact(
            name = "Kasper Jønsson",
            email = "kas@lerpz.com",
        ),
    ),
    tags(
        (name = VOLUMES_TAG, description = "Provision and reclaim the persistent memory volumes attached to agents."),
        (name = RUNTIMES_TAG, description = "Provision and tear down the container runtimes that execute agents."),
        (name = HEALTH_TAG, description = "Monitor Forge health and verify connectivity to the Kubernetes API."),
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
