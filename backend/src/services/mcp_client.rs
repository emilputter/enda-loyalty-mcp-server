use serde_json::{Map, Value};
use std::path::PathBuf;
use tokio::process::Command;

use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ContentBlock, ListToolsResult},
    transport::child_process::TokioChildProcess,
};

pub struct McpClient {
    client: rmcp::service::RunningService<rmcp::RoleClient, ()>,
}

pub struct ToolCallOutcome {
    pub content: String,
    pub is_error: bool,
}

impl McpClient {
    pub async fn connect() -> Self {
        println!("Starting MCP server...");

        let path = std::env::var("MCP_SERVER_PATH").expect("MCP_SERVER_PATH must be set");

        let binary_path = PathBuf::from(&path);
        let mcp_root = binary_path
            .ancestors()
            .nth(3)
            .map(PathBuf::from)
            .expect("MCP_SERVER_PATH must point inside mcp/target/<profile>/");

        let mut command = Command::new(&path);
        command.current_dir(&mcp_root);

        let transport = TokioChildProcess::new(command).expect("Failed to create MCP transport");

        println!("MCP transport created");

        let client = ().serve(transport).await.expect("Failed to connect MCP client");

        println!("Connected to MCP server");

        Self { client }
    }

    pub async fn list_tools(&self) -> ListToolsResult {
        self.client
            .list_tools(None)
            .await
            .expect("Failed to list tools")
    }

    pub async fn call_tool(
        &self,
        name: String,
        arguments: Map<String, Value>,
    ) -> ToolCallOutcome {
        let result = self
            .client
            .call_tool(CallToolRequestParams::new(name).with_arguments(arguments))
            .await
            .expect("Failed to call tool");

        let content = match result.content.first() {
            Some(ContentBlock::Text(text_content)) => text_content.text.clone(),
            _ => "No result".to_string(),
        };

        ToolCallOutcome {
            content,
            is_error: result.is_error.unwrap_or(false),
        }
    }
}
