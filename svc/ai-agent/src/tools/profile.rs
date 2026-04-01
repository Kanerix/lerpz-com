use rig::{completion::ToolDefinition, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::tools::ToolError;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUserProfileArgs {}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
}

pub struct GetUserProfile;

impl Tool for GetUserProfile {
    const NAME: &'static str = "get_user_profile";

    type Error = ToolError;
    type Args = GetUserProfileArgs;
    type Output = UserProfile;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Retrieve the current user's profile information, including their \
                          name and email address."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }),
        }
    }

    #[instrument(skip(self), fields(tool = Self::NAME))]
    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        tracing::debug!("fetching user profile");

        Ok(UserProfile {
            name: "Kasper".to_string(),
            email: "kas@lerpz.com".to_string(),
        })
    }
}
