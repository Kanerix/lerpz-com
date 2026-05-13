use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod delete;
mod edit;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler))
        .routes(routes!(edit::handler))
        .routes(routes!(delete::handler))
}

pub struct ImageAgent {
    pub s3: aws_sdk_s3::Client,
    pub bucket: String,
    // pub mongo: mongodb::Client, - TODO: Add metadata (Mongo or Postgres?)
}
