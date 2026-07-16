use tokio::process::Command;
use std::path::PathBuf;

use rmcp::{
    ServiceExt,
    transport::child_process::TokioChildProcess,
    model::{
        CallToolRequestParams,
        ListToolsResult,
    },
};


pub struct McpClient {
    client: rmcp::service::RunningService<
        rmcp::RoleClient,
        ()
    >,
}


impl McpClient {


    pub async fn connect() -> Self {

        println!("Starting MCP server...");


        let path = std::env::var("MCP_SERVER_PATH")
            .expect("MCP_SERVER_PATH must be set");

        let binary_path = PathBuf::from(&path);
        let mcp_root = binary_path
            .ancestors()
            .nth(3)
            .map(PathBuf::from)
            .expect("MCP_SERVER_PATH must point inside mcp/target/<profile>/");

        let mut command = Command::new(&path);
        command.current_dir(&mcp_root);


        let transport = TokioChildProcess::new(command)
            .expect("Failed to create MCP transport");


        println!("MCP transport created");


        let client = ()
            .serve(transport)
            .await
            .expect("Failed to connect MCP client");


        println!("Connected to MCP server");


        Self {
            client,
        }

    }


    pub async fn list_tools(
        &self
    ) -> ListToolsResult {


        self.client
            .list_tools(None)
            .await
            .expect("Failed to list tools")

    }


 pub async fn call_tool(
    &self,
    name: String,
) -> String {

    
    let result = self.client
        .call_tool(
            CallToolRequestParams::new(name)
        )
        .await
        .expect("Failed to call tool");


    match result.content.first() {

        Some(content) => {
            format!("{:?}", content)
        }

        None => {
            "No result".to_string()
        }
    }
}

}
