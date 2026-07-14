use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::models::Message;
use crate::services::tools::ToolDefinition;

pub enum AIResponse {

    Text(String),

    ToolCall {
        name: String,
        arguments: String,
    }

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


#[derive(Debug, Deserialize)]
struct ToolCall {
    function: FunctionCall,
}


#[derive(Debug, Deserialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}


pub async fn ask_openrouter(
    messages: Vec<Message>,
    tools: Vec<ToolDefinition>
) -> Result<AIResponse, Box<dyn std::error::Error>> {


    let config = Config::load();

    let client = Client::new();


let request = OpenRouterRequest {

    model: "openrouter/free".to_string(),

    messages,

    tools,
};


    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(config.openrouter_key)
        .json(&request)
        .send()
        .await?;


   let text = response.text().await?;

println!("{}", text);

let data: OpenRouterResponse = serde_json::from_str(&text)?;


    let message = &data.choices[0].message;


if let Some(tool_calls) = &message.tool_calls {

    let tool = &tool_calls[0];


    return Ok(
        AIResponse::ToolCall {
            name: tool.function.name.clone(),
            arguments: tool.function.arguments.clone(),
        }
    );
}


Ok(
    AIResponse::Text(
        message
            .content
            .clone()
            .unwrap_or_default()
    )
)
}