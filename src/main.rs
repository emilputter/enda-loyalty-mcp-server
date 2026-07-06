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
   let config = config::Config::load();
    println!("{}", config.redirect_uri);

    //server::start(auth).await;
    let api_client = api_client::ApiClient::new(auth);

match service::get_client_classes(&api_client).await {
    Ok(classes) => {
        println!("Success!");
        println!("{:#?}", classes);
    }

    Err(error) => {
        println!("Error: {}", error);
    }
}

}
