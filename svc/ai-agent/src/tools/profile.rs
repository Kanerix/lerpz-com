use rig::{completion::ToolDefinition, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::tools::ToolError;

/// Arguments accepted by the [`GetUserProfile`] tool.
///
/// This tool takes no parameters — the empty object schema signals to the LLM
/// that it should call the tool without any arguments.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUserProfileArgs {}

/// The user profile returned by [`GetUserProfile`].
#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
}

/// Retrieve the current authenticated user's profile information.
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
