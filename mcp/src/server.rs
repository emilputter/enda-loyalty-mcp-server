use crate::api_client::ApiClient;
use rmcp::ServiceExt;
use rmcp::{tool, tool_router, transport::io::stdio};

//MCP server that shows ENDA backend endpoints as MCP tools
pub struct EndaServer {
    api_client: ApiClient,
}

#[tool_router(server_handler)]
impl EndaServer {
    // Creating a new MCP server instance
    pub fn new(auth: crate::auth::AuthClient) -> Self {
        Self {
            api_client: ApiClient::new(auth),
        }
    }
    // --------------------------------------------------
    // Retrieves the client classes from the ENDA backend
    // --------------------------------------------------
    #[tool(description = "Returns the client classes")]
    async fn enda_list_client_classes(&self) -> String {
        match crate::service::get_client_classes(&self.api_client).await {
            Ok(classes) => serde_json::to_string_pretty(&classes)
                .unwrap_or_else(|error| format!("Serialization Error: {}", error)),

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
        match crate::service::get_rewards(&self.api_client).await {
            Ok(rewards) => serde_json::to_string_pretty(&rewards)
                .unwrap_or_else(|error| format!("Serialization Error: {}", error)),

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
        match crate::service::get_regions(&self.api_client).await {
            Ok(regions) => serde_json::to_string_pretty(&regions)
                .unwrap_or_else(|error| format!("Serialization Error: {}", error)),

            Err(error) => {
                format!("Backend Error: {}", error)
            }
        }
    }
    // --------------------------------------------------------
    // Returns the currently authenticated ENDA user
    // --------------------------------------------------------
    #[tool(description = "Returns the currently authenticated user")]
    async fn enda_current_user(&self) -> String {
        match crate::service::get_current_user(&self.api_client).await {
            Ok(user) => serde_json::to_string_pretty(&user)
                .unwrap_or_else(|error| format!("Serialization Error: {}", error)),

            Err(error) => {
                format!("Backend Error: {}", error)
            }
        }
    }
}

// ----------------------------------------------------------------------
// Starts the MCP server and waits for STDIO connections from MCP clients
// ----------------------------------------------------------------------
pub async fn start(auth: crate::auth::AuthClient) -> Result<(), Box<dyn std::error::Error>> {
    let server = EndaServer::new(auth);

    let service = server.serve(stdio()).await?;

    service.waiting().await?;

    Ok(())
}
