mod api_client;
mod auth;
mod config;
mod models;
mod server;
mod service;
use dotenvy::dotenv;
#[tokio::main]

// Entry of the ENDA MCP server
async fn main() {
    dotenv().ok();

    eprintln!("Starting ENDA MCP Server");

    let mut auth = auth::AuthClient::new();

    auth.login().await.unwrap();

    server::start(auth).await;
}
