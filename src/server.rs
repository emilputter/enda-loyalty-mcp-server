use rmcp::ServiceExt;
use rmcp::{
    tool,
    tool_router,
    transport::io::stdio,
};

use sqlx::{Pool, Postgres};

pub struct EndaServer {
    pub pool: Pool<Postgres>,
}
#[tool_router(server_handler)]
impl EndaServer {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {pool}
    }

#[tool(description = "Returns the client classes")]
async fn enda_list_client_classes(&self) -> String{
    "Hello from ENDA MCP".to_string()
}
}

pub async fn start(pool: Pool<Postgres>) {

    let server = EndaServer::new(pool);

    // for debugging
    //eprintln!("testing MCP server");

    let service = server
    .serve(stdio())
    .await
    .unwrap();

    service
    .waiting()
    .await
    .unwrap();



}
