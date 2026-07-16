use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::models::{Message, ToolCall};
use crate::services::tools::ToolDefinition;

pub enum AIResponse {
    Text(String),
    ToolCalls {
        assistant_message: Message,
        calls: Vec<ToolCall>,
    },
}

#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
    tools: Vec<ToolDefinition>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Debug, Deserialize)]
struct MessageResponse {
    content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

/// Requests the next model response. Tool calls are returned intact so the
/// caller can execute every call and pass their results back to the model.
pub async fn ask_openrouter(
    messages: Vec<Message>,
    tools: Vec<ToolDefinition>,
) -> Result<AIResponse, Box<dyn std::error::Error>> {
    let config = Config::load();
    let request = OpenRouterRequest {
        model: "tencent/hy3:free".to_string(),
        messages,
        tools,
    };

    let response = Client::new()
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(config.openrouter_key)
        .json(&request)
        .send()
        .await?
        .error_for_status()?;
    let data: OpenRouterResponse = response.json().await?;
    let message = data
        .choices
        .into_iter()
        .next()
        .ok_or("OpenRouter returned no choices")?
        .message;
    let assistant_message = Message {
        role: "assistant".to_string(),
        content: message.content.clone().unwrap_or_default(),
        tool_calls: message.tool_calls.clone(),
        tool_call_id: None,
    };

    match message.tool_calls {
        Some(calls) if !calls.is_empty() => Ok(AIResponse::ToolCalls {
            assistant_message,
            calls,
        }),
        _ => Ok(AIResponse::Text(message.content.unwrap_or_default())),
    }
}
