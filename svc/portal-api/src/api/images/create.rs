use async_openai::types::{CreateImageRequestArgs, Image, ImageModel, ImageQuality, ImageSize};
use axum::{Json, extract::State};
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, oapi::IMAGES_TAG, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ImageRequest {
    /// What model to use.
    model: Option<String>,
    /// Prompt that is sent to the model.
    prompt: String,
    /// The amount of images to generate.
    ///
    /// This will default to 1 image if not provided.
    amount: Option<u8>,
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
    token: AzureAccessToken,
    State(state): State<AppState>,
    Json(body): Json<ImageRequest>,
) -> HandlerResult<Json<ImageResponse>> {
    let model = ImageModel::Other(
        body.model
            .as_deref()
            .unwrap_or(&CONFIG.DEFAULT_IMAGE_MODEL)
            .to_string(),
    );

    let mut request_builder = CreateImageRequestArgs::default();

    request_builder
        .model(model)
        .prompt(body.prompt)
        .n(body.amount.unwrap_or(1))
        .quality(ImageQuality::Low)
        .size(ImageSize::S1024x1024);

    if let Some(email) = token.email {
        request_builder.user(email);
    };

    let request = request_builder.build().unwrap();

    let client = state.openai.read().await;
    let response = client.images().create(request).await?;

    let images = response
        .data
        .iter()
        .filter_map(|img| match img.as_ref() {
            Image::B64Json { b64_json, .. } => {
                dbg!(&img);
                Some(b64_json.to_string())
            }
            _ => None,
        })
        .collect();

    Ok(Json(ImageResponse { images }))
}
