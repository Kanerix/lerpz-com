use utoipa::OpenApi;

const HEALTH_TAG: &str = "image";
const IMAGE_TAG: &str = "image";
const CHAT_TAG: &str = "chat";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = CHAT_TAG, description = "Chat API endpoints"),
        (name = IMAGE_TAG, description = "Image API endpoints"),
        (name = HEALTH_TAG, description = "Health API endpoints"),
    )
)]
pub(crate) struct ApiDoc;
