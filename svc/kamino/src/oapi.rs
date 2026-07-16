use lerpz_axum::oapi::EntraAuth;
use utoipa::{Modify, OpenApi};

use crate::config::CONFIG;

pub(crate) const CHATS_TAG: &str = "chats";
pub(crate) const IMAGES_TAG: &str = "images";
pub(crate) const VIDEOS_TAG: &str = "videos";
pub(crate) const ENHANCE_TAG: &str = "enhance";
pub(crate) const MODELS_TAG: &str = "models";
pub(crate) const GROUPS_TAG: &str = "groups";
pub(crate) const ORGS_TAG: &str = "orgs";
pub(crate) const AGENTS_TAG: &str = "agents";
pub(crate) const SESSIONS_TAG: &str = "sessions";
pub(crate) const HEALTH_TAG: &str = "health";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lerpz AI — API references",
        description = "The Lerpz AI API for building AI-powered experiences. \
            Manage organizations, groups, agents, and sessions; stream chat \
            completions and generate images in real time; and discover the \
            models available to your organization.",
        contact(
            name = "Kasper Jønsson",
            email = "kas@lerpz.com",
        ),
    ),
    tags(
        (name = CHATS_TAG, description = "Manage AI conversations and stream responses via Server-Sent Events."),
        (name = IMAGES_TAG, description = "Generate and edit images using AI models with real-time streaming previews."),
        (name = VIDEOS_TAG, description = "Generate videos using AI models and analyse them for titles and tags."),
        (name = ENHANCE_TAG, description = "Refine raw prompts into richer, model-ready prompts for chat, image, and video generation."),
        (name = MODELS_TAG, description = "Discover and manage the AI models available to your organization."),
        (name = GROUPS_TAG, description = "Manage user groups and their access to organizational resources."),
        (name = ORGS_TAG, description = "Manage organizations, their members, and top-level settings."),
        (name = AGENTS_TAG, description = "Manage AI agents and their configuration."),
        (name = SESSIONS_TAG, description = "Manage agent sessions and their lifecycle."),
        (name = HEALTH_TAG, description = "Monitor API health and verify connectivity to backing services."),
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
