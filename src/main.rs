mod models;
mod config;
mod server;
mod service;
mod api_client;
mod auth;
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
