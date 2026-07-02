mod models;
mod server;
mod service;
use dotenvy::dotenv;
#[tokio::main]

// Entry of the ENDA MCP server
async fn main() {

    dotenv().ok();

    eprintln!("Starting ENDA MCP Server");

    server::start().await;
}
