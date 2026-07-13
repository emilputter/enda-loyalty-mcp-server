use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;


#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<Message>,
}


#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
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
    content: String,
}


pub async fn ask_openrouter(
    user_message: String
) -> Result<String, reqwest::Error> {


    let config = Config::load();

    let client = Client::new();


    let request = OpenRouterRequest {

        model: "openrouter/free".to_string(),

        messages: vec![
            Message {
                role: "user".to_string(),
                content: user_message,
            }
        ],
    };


    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .bearer_auth(config.openrouter_key)
        .json(&request)
        .send()
        .await?;


    let data: OpenRouterResponse = response
        .json()
        .await?;


    Ok(
        data.choices[0]
            .message
            .content
            .clone()
    )
}