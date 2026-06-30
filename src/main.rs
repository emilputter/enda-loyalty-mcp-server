mod database;
mod models;

mod server;
#[tokio::main]


 async fn main() {

    eprintln!("Starting ENDA MCP Server");

    let pool = database::connect().await;

    server::start(pool).await;

}

