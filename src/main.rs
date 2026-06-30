mod database;
mod models;
mod tools;
mod server;
#[tokio::main]


 async fn main() {

    println!("Starting ENDA MCP Server");

    let pool = database::connect().await;

    server::start(pool).await;

}

