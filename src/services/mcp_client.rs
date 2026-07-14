use tokio::process::Command;

use rmcp::{
    ServiceExt,
    transport::child_process::TokioChildProcess,
    model::CallToolRequestParams,
};


pub async fn test_mcp_connection() {

    println!("Starting MCP server...");


    let path = std::env::var("MCP_SERVER_PATH")
        .expect("MCP_SERVER_PATH must be set");


    let command = Command::new(path);


    let transport = TokioChildProcess::new(command)
        .expect("Failed to create MCP transport");


    println!("MCP transport created");


    let client = ()
        .serve(transport)
        .await
        .expect("Failed to connect MCP client");


    println!("Connected to MCP server");


    let tools = client
        .list_tools(None)
        .await
        .expect("Failed to list tools");


    println!("Available tools:");
println!("{:#?}", tools);


let result = client
    .call_tool(
        CallToolRequestParams::new(
            "enda_current_user"
        )
    )
    .await
    .expect("Failed to call tool");


println!("Tool result:");
println!("{:#?}", result);

}