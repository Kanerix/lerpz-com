use utoipa::{
    OpenApi,
    openapi::{
        SecurityRequirement,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

pub(crate) const CHATS_TAG: &str = "chats";
pub(crate) const IMAGES_TAG: &str = "images";
pub(crate) const VIDEOS_TAG: &str = "videos";
pub(crate) const ENHANCE_TAG: &str = "enhance";
pub(crate) const MODELS_TAG: &str = "models";
pub(crate) const GROUPS_TAG: &str = "groups";
pub(crate) const AGENTS_TAG: &str = "agents";

pub(crate) const SESSIONS_TAG: &str = "sessions";
pub(crate) const SETTINGS_TAG: &str = "settings";
pub(crate) const HEALTH_TAG: &str = "health";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Lerpz AI API references",
        version = crate::version::VERSION,
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
        (name = AGENTS_TAG, description = "Manage private agents and their persistent memory."),
        (name = SESSIONS_TAG, description = "Manage agent sessions and their lifecycle."),
        (name = SETTINGS_TAG, description = "Manage the authenticated user's account settings and preferences."),
        (name = HEALTH_TAG, description = "Monitor API health and verify connectivity to backing services."),
    )
)]
pub(crate) struct ApiDoc;

/// Build the base OpenAPI document without loading service configuration.
pub(crate) fn api_doc() -> utoipa::openapi::OpenApi {
    let mut openapi = ApiDoc::openapi();

    openapi
        .components
        .get_or_insert_with(Default::default)
        .add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    openapi.security = Some(vec![SecurityRequirement::new(
        "bearer_auth",
        Vec::<String>::new(),
    )]);

    openapi
}
