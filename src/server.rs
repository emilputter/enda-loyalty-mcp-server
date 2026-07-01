use rmcp::ServiceExt;
use rmcp::{tool, tool_router, transport::io::stdio};
use serde_json;
use sqlx::{Pool, Postgres};

//MCP server that shows ENDA backend endpoints as MCP tools
pub struct EndaServer {
    pub pool: Pool<Postgres>,
}
#[tool_router(server_handler)]
impl EndaServer {
    // Creating a new MCP server instance
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    // --------------------------------------------------
    // Retrieves the client classes from the ENDA backend
    // --------------------------------------------------
    #[tool(description = "Returns the client classes")]
    async fn enda_list_client_classes(&self) -> String {
        match crate::service::get_client_classes().await {
            Ok(classes) => serde_json::to_string_pretty(&classes).unwrap(),

            Err(error) => {
                format!("Backend Error: {}", error)
            }
        }
    }
    // -------------------------------------------
    // Retrieves all rewards from the EDNA backend
    // -------------------------------------------
    #[tool(description = "Returns all rewards")]
    async fn enda_list_rewards(&self) -> String {
        match crate::service::get_rewards().await {
            Ok(rewards) => serde_json::to_string_pretty(&rewards).unwrap(),

            Err(error) => {
                format!("Backend Error: {}", error)
            }
        }
    }
    // -------------------------------------------
    // Retrieves all regions from the ENDA backend
    // -------------------------------------------
    #[tool(description = "Returns all regions")]
    async fn enda_list_regions(&self) -> String {
        match crate::service::get_regions().await {
            Ok(regions) => serde_json::to_string_pretty(&regions).unwrap(),

            Err(error) => {
                format!("Backend Error: {}", error)
            }
        }
    }
}

// ----------------------------------------------------------------------
// Starts the MCP server and waits for STDIO connections from MCP clients
// ----------------------------------------------------------------------
pub async fn start(pool: Pool<Postgres>) {
    let server = EndaServer::new(pool);

    let service = server.serve(stdio()).await.unwrap();

    service.waiting().await.unwrap();
}
