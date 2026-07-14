use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct ToolDefinition {

    pub r#type: String,

    pub function: FunctionDefinition,

}


#[derive(Debug, Serialize)]
pub struct FunctionDefinition {

    pub name: String,

    pub description: String,

    pub parameters: serde_json::Value,

}