use utoipa::OpenApi;

pub(crate) const CHATS_TAG: &str = "chat";
pub(crate) const IMAGES_TAG: &str = "image";
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
