mod config;
mod models;
mod services;

use crate::models::{Message, ToolActivity};
use axum::{Json, Router, extract::State, routing::post};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use services::openrouter::AIResponse;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

const MAX_TOOL_ROUNDS: usize = 8;
const MAX_TOOL_ERRORS: usize = 2;

#[derive(Clone)]
struct AppState {
    mcp: Arc<services::mcp_client::McpClient>,
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
    tool_activity: Vec<ToolActivity>,
}

async fn chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Json<ChatResponse> {
    let tools = services::tools::convert_mcp_tools(state.mcp.list_tools().await.tools);
    let mut messages = payload.messages;
    let mut activity = Vec::new();
    let mut tool_errors = 0;

    for _ in 0..MAX_TOOL_ROUNDS {
        let response =
            match services::openrouter::ask_openrouter(messages.clone(), tools.clone()).await {
                Ok(response) => response,
                Err(error) => {
                    return Json(ChatResponse {
                        response: format!("I could not contact the AI service: {error}"),
                        tool_activity: activity,
                    });
                }
            };

        match response {
            AIResponse::Text(response) => {
                return Json(ChatResponse {
                    response,
                    tool_activity: activity,
                });
            }
            AIResponse::ToolCalls {
                assistant_message,
                calls,
            } => {
                messages.push(assistant_message);

                for call in calls {
                    let mut arguments = match serde_json::from_str::<Map<String, Value>>(
                        &call.function.arguments,
                    ) {
                        Ok(arguments) => arguments,
                        Err(error) => {
                            let result =
                                format!("The model supplied invalid JSON arguments: {error}");
                            activity.push(ToolActivity {
                                name: call.function.name.clone(),
                                arguments: Value::Null,
                                result: result.clone(),
                            });
                            messages.push(tool_result_message(call.id, result));
                            continue;
                        }
                    };
                    normalize_json_body(&mut arguments);
                    let outcome = state
                        .mcp
                        .call_tool(call.function.name.clone(), arguments.clone())
                        .await;
                    let result = outcome.content;
                    activity.push(ToolActivity {
                        name: call.function.name.clone(),
                        arguments: Value::Object(arguments),
                        result: result.clone(),
                    });
                    messages.push(tool_result_message(call.id, result));

                    if outcome.is_error {
                        tool_errors += 1;
                        if tool_errors >= MAX_TOOL_ERRORS {
                            return Json(ChatResponse {
                                response: "The requested operation could not be completed after two API errors. Review the tool details for the API validation message and try again with corrected values.".to_string(),
                                tool_activity: activity,
                            });
                        }
                    } else {
                        tool_errors = 0;
                    }
                }
            }
        }
    }

    Json(ChatResponse {
        response: "I stopped after too many tool steps. Please try a more specific request."
            .to_string(),
        tool_activity: activity,
    })
}

/// Some models serialize an object-valued `body` argument as a JSON string.
/// The MCP schema requires the object itself, so repair that representation
/// before forwarding the request to the ENDA API.
fn normalize_json_body(arguments: &mut Map<String, Value>) {
    let Some(Value::String(body)) = arguments.get("body") else {
        return;
    };
    let Ok(parsed) = serde_json::from_str::<Value>(body) else {
        return;
    };
    if parsed.is_object() || parsed.is_array() {
        arguments.insert("body".to_string(), parsed);
    }
}

fn tool_result_message(tool_call_id: String, content: String) -> Message {
    Message {
        role: "tool".to_string(),
        content,
        tool_calls: None,
        tool_call_id: Some(tool_call_id),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mcp = services::mcp_client::McpClient::connect().await;
    let app = Router::new()
        .route("/chat", post(chat))
        .with_state(AppState { mcp: Arc::new(mcp) })
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("AI backend running on port 8080");
    axum::serve(listener, app).await.unwrap();
}
