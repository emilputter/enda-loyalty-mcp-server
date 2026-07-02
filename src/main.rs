mod models;
mod server;
mod service;
#[tokio::main]

// Entry of the ENDA MCP server
async fn main() {
    eprintln!("Starting ENDA MCP Server");

    server::start().await;
}
