use anyhow::Result;
use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};
use serde_json::{Value, json};

pub fn tool_definitions() -> Vec<ChatCompletionTools> {
    vec![
        ChatCompletionTools::Function(ChatCompletionTool {
            function: FunctionObjectArgs::default()
                .name("search_knowledge_base")
                .description("Search the knowledge base for relevant documents given a query")
                .parameters(json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The search query to find relevant documents"
                        },
                        "top_k": {
                            "type": "integer",
                            "description": "Number of results to return (default 5)"
                        }
                    },
                    "required": ["query"],
                    "additionalProperties": false
                }))
                .strict(true)
                .build()
                .expect("Failed to build search_knowledge_base tool"),
        }),
        ChatCompletionTools::Function(ChatCompletionTool {
            function: FunctionObjectArgs::default()
                .name("get_user_profile")
                .description("Retrieve the current user's profile information")
                .parameters(json!({
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }))
                .strict(true)
                .build()
                .expect("Failed to build get_user_profile tool"),
        }),
    ]
}

pub async fn execute_tool(name: &str, args: &Value) -> Result<Value> {
    match name {
        "search_knowledge_base" => {
            let query = args["query"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Missing 'query' argument"))?;
            let top_k = args["top_k"].as_u64().unwrap_or(5) as usize;

            Ok(json!({
                "results": [
                    {"title": "Example doc", "content": "This is a retrieved document."}
                ]
            }))
        }
        "get_user_profile" => Ok(json!({
            "name": "Kasper",
            "email": "kas@lerpz.com"
        })),
        _ => Err(anyhow::anyhow!("Unknown tool: {name}")),
    }
}
