mod config;
mod services;
mod models;

use services::openrouter::AIResponse;
use crate::models::Message;
use std::sync::Arc;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use axum::{
    routing::post,
    Json,
    Router,
    extract::State,
};

use serde::{Deserialize, Serialize};

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
}


async fn chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>
) -> Json<ChatResponse> {

    println!("Received messages: {:?}", payload.messages);


    let mcp_tools = state
        .mcp
        .list_tools()
        .await;


    let tools = services::tools::convert_mcp_tools(
        mcp_tools.tools
    );


    let response = services::openrouter::ask_openrouter(
    payload.messages,
    tools,
)
.await
.unwrap();


match response {

    AIResponse::Text(text) => {

        Json(ChatResponse {
            response: text,
        })

    }


    AIResponse::ToolCall { name, arguments } => {

    println!(
        "AI requested tool: {} with arguments {}",
        name,
        arguments
    );


    let result = state
        .mcp
        .call_tool(name)
        .await;


    Json(ChatResponse {
        response: result,
    })

}

}
}


#[tokio::main]
async fn main() {

    dotenv().ok();

    println!(
    "MCP PATH = {:?}",
    std::env::var("MCP_SERVER_PATH")
);

    let mcp = services::mcp_client::McpClient::connect().await;

let state = AppState {
    mcp: Arc::new(mcp),
};


let app = Router::new()
    .route("/chat", post(chat))
    .with_state(state)
    .layer(CorsLayer::permissive());


    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:8080"
    )
    .await
    .unwrap();


    println!("AI backend running on port 8080");


    axum::serve(
        listener,
        app
    )
    .await
    .unwrap();
}
