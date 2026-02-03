use async_openai::types::{CreateImageRequestArgs, Image, ImageModel};
use axum::{Json, extract::State};
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, oapi::IMAGES_TAG, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ImageRequest {
    model: Option<String>,
    prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ImageResponse {
    images: Vec<String>,
}

#[utoipa::path(
    method(post),
    path = "/",
    tag = IMAGES_TAG,
    summary = "Create a new image",
)]
#[axum::debug_handler]
pub async fn handler(
    _: AzureAccessToken,
    State(state): State<AppState>,
    Json(body): Json<ImageRequest>,
) -> HandlerResult<Json<ImageResponse>> {
    let model = ImageModel::Other(
        body.model
            .as_deref()
            .unwrap_or(&CONFIG.DEFAULT_IMAGE_MODEL)
            .to_string(),
    );

    let request = CreateImageRequestArgs::default()
        .model(model)
        .prompt(body.prompt)
        .build()?;
    let client = state.openai.read().await;
    let response = client.images().create(request).await?;

    let images = response
        .data
        .iter()
        .filter_map(|img| match img.as_ref() {
            Image::B64Json { b64_json, .. } => Some(b64_json.to_string()),
            _ => None,
        })
        .collect();

    Ok(Json(ImageResponse { images }))
}
