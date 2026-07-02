mod models;
mod config;
mod server;
mod service;
mod api_client;
use dotenvy::dotenv;
#[tokio::main]

// Entry of the ENDA MCP server
async fn main() {

    dotenv().ok();

    eprintln!("Starting ENDA MCP Server");

    server::start().await;
}
