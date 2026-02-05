use axum::Json;
use lerpz_axum::error::HandlerResult;
use serde::Serialize;
use utoipa::ToSchema;

use crate::oapi::MODELS_TAG;

pub type Response = Json<Models>;

#[derive(Serialize, ToSchema)]
pub struct Models {
    models: Vec<Model>,
}

#[derive(Serialize, ToSchema)]
pub struct Model {
    name: String,
    slug: String,
    family: String,
}

#[utoipa::path(
    method(get),
    path = "/",
    tag = MODELS_TAG,
    summary = "Get available models",
    responses(
        (status = OK, description = "Success", body = Models)
    )
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<Response> {
    let models = vec![
        Model {
            name: "gpt-image-1".into(),
            slug: "@azure/gpt-image-1".into(),
            family: "openai".into(),
        },
        Model {
            name: "gemini-2.5-flash-image".into(),
            slug: "@google-ai/gemini-2.5-flash-image".into(),
            family: "google-ai".into(),
        },
    ];

    Ok(Json(Models { models }))
}
