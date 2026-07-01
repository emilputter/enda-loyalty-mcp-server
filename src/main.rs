mod database;
mod models;
mod server;
mod service;
#[tokio::main]

// Entry of the ENDA MCP server
async fn main() {
    eprintln!("Starting ENDA MCP Server");

    // Makes the connection to the PostgreSQL database
    let pool = database::connect().await;

    // Starts the MCP server
    server::start(pool).await;
}
