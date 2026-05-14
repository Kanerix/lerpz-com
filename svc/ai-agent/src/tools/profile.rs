use reqwest::Client;
use rig_core::{completion::ToolDefinition, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::tools::ToolError;

const MS_GRAPH_ME_URL: &str = "https://graph.microsoft.com/v1.0/me";

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUserProfileArgs {}

/// Raw response shape from `GET https://graph.microsoft.com/v1.0/me`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MsGraphMe {
    /// The user's display name from Entra ID.
    display_name: String,
    /// Present for cloud-only and synced accounts; absent for some
    /// guest/federated types.
    mail: Option<String>,
    /// Always present – used as a fallback when `mail` is `None`.
    user_principal_name: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
}

/// Tool that fetches the authenticated user's profile from Microsoft Graph.
///
/// The access token is captured at construction time and is **never** forwarded
/// to the LLM – only the tool's JSON schema (empty arg object) is sent.
pub struct GetUserProfile {
    client: Client,
    token: String,
}

impl GetUserProfile {
    pub fn new(client: Client, token: impl Into<String>) -> Self {
        Self {
            client,
            token: token.into(),
        }
    }
}

impl Tool for GetUserProfile {
    const NAME: &'static str = "get_user_profile";

    type Error = ToolError;
    type Args = GetUserProfileArgs;
    type Output = UserProfile;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Retrieve the current user's profile information, \
                including their name and email address."
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
        tracing::debug!("fetching user profile from Microsoft Graph");

        let graph_me: MsGraphMe = self
            .client
            .get(MS_GRAPH_ME_URL)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| ToolError::ProfileFailed(e.to_string()))?
            .error_for_status()
            .map_err(|e| ToolError::ProfileFailed(e.to_string()))?
            .json()
            .await
            .map_err(|e| ToolError::ProfileFailed(e.to_string()))?;

        tracing::debug!(name = %graph_me.display_name, "user profile fetched");

        Ok(UserProfile {
            // The user's display name from Entra ID.
            name: graph_me.display_name,
            // `mail` can be absent for certain account types; fall back to UPN.
            email: graph_me.mail.unwrap_or(graph_me.user_principal_name),
        })
    }
}
