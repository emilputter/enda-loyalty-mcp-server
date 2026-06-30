mod database;
mod models;
mod tools;
mod server;
#[tokio::main]


 async fn main() {

   let classes = tools::get_client_classes();

    println!(" ENDA Loyalty MCP Server");
    println!("Starting server...");
   
    database::connect().await;
    server::start().await;

    for class in classes {
      println!("Name: {}", class.name);
      println!("Max Scpre: {}", class.max_score)
    }

}

