use serde::Serialize;
use rmcp::model::Tool;


#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {

    pub r#type: String,

    pub function: FunctionDefinition,

}


#[derive(Debug, Clone, Serialize)]
pub struct FunctionDefinition {

    pub name: String,

    pub description: String,

    pub parameters: serde_json::Value,

}

pub fn convert_mcp_tools(
    tools: Vec<Tool>
) -> Vec<ToolDefinition> {

    tools
        .into_iter()
        .map(|tool| {

            ToolDefinition {
                r#type: "function".to_string(),

                function: FunctionDefinition {
                    name: tool.name.to_string(),

                    description: tool
                    .description
                    .unwrap_or_default()
                    .to_string(),

                    parameters: serde_json::to_value(
                        tool.input_schema
                    )
                    .unwrap(),
                }
            }

        })
        .collect()
}
