use async_openai::Client;
use async_openai::types::chat::{
    ChatCompletionMessageToolCalls, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestToolMessage, ChatCompletionRequestUserMessage,
    CreateChatCompletionRequestArgs,
};
use tracing::trace;

use crate::error::{Error, Result};
use crate::portkey::PortkeyConfig;
use crate::tools::{execute_tool, tool_definitions};

const MAX_TOOL_ROUNDS: usize = 10;

pub struct Agent {
    client: Client<PortkeyConfig>,
    model: String,
    system_prompt: String,
}

impl Agent {
    pub fn new(config: PortkeyConfig, model: &str, system_prompt: &str) -> Self {
        Self {
            client: Client::with_config(config),
            model: model.to_string(),
            system_prompt: system_prompt.to_string(),
        }
    }

    /// Run the agent: send user input, handle tool calls in a loop, return the
    /// final text response.
    pub async fn run(&self, user_input: &str) -> Result<String> {
        let mut messages: Vec<ChatCompletionRequestMessage> = vec![
            ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                content: self.system_prompt.clone().into(),
                ..Default::default()
            }),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
                content: user_input.to_string().into(),
                ..Default::default()
            }),
        ];

        let tools = tool_definitions();

        for round in 0..MAX_TOOL_ROUNDS {
            trace!(round, "Sending chat completion request");

            let tools = tools.clone();
            let request = CreateChatCompletionRequestArgs::default()
                .model(&self.model)
                .messages(messages.clone())
                .tools(tools)
                .build()?;

            let response = self.client.chat().create(request).await?;
            let choice = response
                .choices
                .into_iter()
                .next()
                .ok_or_else(|| anyhow::anyhow!("No choices in response"))?;

            let message = choice.message;

            if let Some(ref tool_calls) = message.tool_calls {
                trace!(count = tool_calls.len(), "Model requested tool calls");

                let assistant_msg = ChatCompletionRequestAssistantMessageArgs::default()
                    .tool_calls(tool_calls.clone())
                    .build()?;
                messages.push(assistant_msg.into());

                for tool_call in tool_calls {
                    if let ChatCompletionMessageToolCalls::Function(func) = tool_call {
                        let args = serde_json::from_str(&func.function.arguments)?;

                        trace!(tool = %func.function.name, "Executing tool");
                        let result = execute_tool(&func.function.name, &args).await?;

                        messages.push(ChatCompletionRequestMessage::Tool(
                            ChatCompletionRequestToolMessage {
                                content: result.to_string().into(),
                                tool_call_id: func.id.clone(),
                            },
                        ));
                    }
                }

                continue;
            }

            let content = message
                .content
                .ok_or_else(|| anyhow::anyhow!("No content in final response"))?;
            return Ok(content.to_string());
        }

        Err(Error::Agent(format!(
            "Agent exceeded max tool rounds ({MAX_TOOL_ROUNDS})"
        )))
    }
}
