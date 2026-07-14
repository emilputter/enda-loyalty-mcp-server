mod config;
mod services;
mod models;


use crate::models::Message;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use axum::{
    routing::post,
    Json,
    Router,
};

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
struct ChatRequest {
    messages: Vec<Message>,
}



#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
}


async fn chat(
    Json(payload): Json<ChatRequest>
) -> Json<ChatResponse> {

    println!("Received messages: {:?}", payload.messages);


    let response = services::openrouter::ask_openrouter(
    payload.messages,
    Vec::new(),
)
.await
.unwrap();


    Json(ChatResponse {
        response,
    })
}


#[tokio::main]
async fn main() {

    dotenv().ok();

    println!(
    "MCP PATH = {:?}",
    std::env::var("MCP_SERVER_PATH")
);

    services::mcp_client::test_mcp_connection().await;
    
    let app = Router::new()
        .route("/chat", post(chat))
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