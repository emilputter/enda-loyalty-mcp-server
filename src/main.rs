mod api_client;
mod auth;
mod config;
mod models;
mod server;
mod service;
use dotenvy::dotenv;
#[tokio::main]

// Entry of the ENDA MCP server
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    eprintln!("Starting ENDA MCP Server");

    let mut auth = auth::AuthClient::new();

    auth.login().await?;

    let config = config::Config::load();

    println!("{}", config.redirect_uri);

    server::start(auth).await?;

    Ok(())
}
