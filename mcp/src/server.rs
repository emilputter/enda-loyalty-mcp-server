use crate::api_client::ApiClient;
use crate::config::Config;
use rmcp::{
    ErrorData, ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResult, ContentBlock, ListToolsResult, ServerCapabilities,
        ServerInfo, Tool,
    },
    transport::io::stdio,
};
use serde_json::{Map, Value, json};
use std::collections::{BTreeSet, HashMap};

/// An operation exposed by the My Enda OpenAPI document.
#[derive(Clone)]
pub(crate) struct ApiOperation {
    tool: Tool,
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) body_media_type: Option<String>,
    pub(crate) parameters: Vec<ApiParameter>,
}

#[derive(Clone)]
pub(crate) struct ApiParameter {
    pub(crate) name: String,
    pub(crate) location: String,
}

/// Builds MCP tools from the authoritative My Enda OpenAPI document.
struct ApiCatalog {
    operations: HashMap<String, ApiOperation>,
}

impl ApiCatalog {
    async fn download(config: &Config) -> Result<Self, String> {
        let document = reqwest::Client::new()
            .get(&config.openapi_url)
            .send()
            .await
            .map_err(|error| format!("could not download OpenAPI document: {error}"))?
            .error_for_status()
            .map_err(|error| format!("OpenAPI endpoint returned an error: {error}"))?
            .json::<Value>()
            .await
            .map_err(|error| format!("OpenAPI document is not valid JSON: {error}"))?;

        Self::from_document(document)
    }

    fn from_document(document: Value) -> Result<Self, String> {
        let paths = document["paths"]
            .as_object()
            .ok_or("OpenAPI document has no paths object")?;
        let components = document
            .pointer("/components/schemas")
            .cloned()
            .unwrap_or_else(|| json!({}));
        let mut operations = HashMap::new();

        for (path, item) in paths {
            let Some(item) = item.as_object() else {
                continue;
            };
            for method in ["get", "post", "put", "patch", "delete"] {
                let Some(operation) = item.get(method).and_then(Value::as_object) else {
                    continue;
                };

                let name = tool_name(method, path);
                let parameters = operation
                    .get("parameters")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(|parameter| {
                        Some(ApiParameter {
                            name: parameter.get("name")?.as_str()?.to_owned(),
                            location: parameter.get("in")?.as_str()?.to_owned(),
                        })
                    })
                    .collect::<Vec<_>>();

                let (input_schema, body_media_type) = input_schema(operation, &components)?;
                let summary = operation
                    .get("summary")
                    .and_then(Value::as_str)
                    .unwrap_or("Call My Enda API");
                let description = operation
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .replace('\n', " ");
                let description = format!(
                    "{method_upper} {path} — {summary}. {description}",
                    method_upper = method.to_uppercase()
                );
                let schema = input_schema
                    .as_object()
                    .cloned()
                    .ok_or("generated input schema is not an object")?;

                operations.insert(
                    name.clone(),
                    ApiOperation {
                        tool: Tool::new(name.clone(), description, schema),
                        method: method.to_uppercase(),
                        path: path.clone(),
                        body_media_type,
                        parameters,
                    },
                );
            }
        }

        if operations.is_empty() {
            return Err("OpenAPI document does not contain any HTTP operations".to_owned());
        }
        Ok(Self { operations })
    }

    fn tools(&self) -> Vec<Tool> {
        let mut tools = self
            .operations
            .values()
            .map(|operation| operation.tool.clone())
            .collect::<Vec<_>>();
        tools.sort_by(|left, right| left.name.cmp(&right.name));
        tools
    }
}

/// Converts OpenAPI request inputs into an MCP JSON Schema object.
///
/// The request body is kept under `body`. `$ref`s are rewritten to local `$defs`,
/// so clients receive the exact schema published by My Enda, including enums,
/// required fields, nested objects, and recursive JEL expressions.
fn input_schema(
    operation: &Map<String, Value>,
    components: &Value,
) -> Result<(Value, Option<String>), String> {
    let mut properties = Map::new();
    let mut required = Vec::new();
    let mut references = BTreeSet::new();

    for parameter in operation
        .get("parameters")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let name = parameter
            .get("name")
            .and_then(Value::as_str)
            .ok_or("OpenAPI parameter has no name")?;
        let schema = parameter
            .get("schema")
            .cloned()
            .unwrap_or_else(|| json!({}));
        collect_references(&schema, &mut references);
        properties.insert(name.to_owned(), rewrite_references(schema));
        if parameter
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            required.push(Value::String(name.to_owned()));
        }
    }

    let mut body_media_type = None;
    if let Some(request_body) = operation.get("requestBody").and_then(Value::as_object) {
        let content = request_body
            .get("content")
            .and_then(Value::as_object)
            .ok_or("OpenAPI request body has no content")?;
        let (media_type, media) = content
            .iter()
            .find(|(media_type, _)| media_type.as_str() == "application/json")
            .or_else(|| content.iter().next())
            .ok_or("OpenAPI request body content is empty")?;
        let schema = media.get("schema").cloned().unwrap_or_else(|| json!({}));
        collect_references(&schema, &mut references);
        properties.insert("body".to_owned(), rewrite_references(schema));
        if request_body
            .get("required")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            required.push(Value::String("body".to_owned()));
        }
        body_media_type = Some(media_type.clone());
    }

    let definitions = required_definitions(&references, components)?;
    let mut schema = json!({
        "type": "object",
        "properties": properties,
        "additionalProperties": false,
    });
    if !required.is_empty() {
        schema["required"] = Value::Array(required);
    }
    if !definitions.is_empty() {
        schema["$defs"] = Value::Object(definitions);
    }
    Ok((schema, body_media_type))
}

fn collect_references(value: &Value, references: &mut BTreeSet<String>) {
    match value {
        Value::Object(object) => {
            if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
                if let Some(name) = reference.strip_prefix("#/components/schemas/") {
                    references.insert(name.to_owned());
                }
            }
            for value in object.values() {
                collect_references(value, references);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_references(value, references);
            }
        }
        _ => {}
    }
}

fn required_definitions(
    references: &BTreeSet<String>,
    components: &Value,
) -> Result<Map<String, Value>, String> {
    let mut pending = references.iter().cloned().collect::<Vec<_>>();
    let mut completed = BTreeSet::new();
    let mut definitions = Map::new();
    while let Some(name) = pending.pop() {
        if !completed.insert(name.clone()) {
            continue;
        }
        let schema = components
            .get(&name)
            .cloned()
            .ok_or_else(|| format!("OpenAPI schema reference {name} is missing"))?;
        let mut nested = BTreeSet::new();
        collect_references(&schema, &mut nested);
        pending.extend(
            nested
                .into_iter()
                .filter(|nested| !completed.contains(nested)),
        );
        definitions.insert(name, rewrite_references(schema));
    }
    Ok(definitions)
}

fn rewrite_references(value: Value) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .into_iter()
                .map(|(key, value)| {
                    let value = if key == "$ref" {
                        match value
                            .as_str()
                            .and_then(|reference| reference.strip_prefix("#/components/schemas/"))
                        {
                            Some(name) => Value::String(format!("#/$defs/{name}")),
                            None => value,
                        }
                    } else {
                        rewrite_references(value)
                    };
                    (key, value)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.into_iter().map(rewrite_references).collect()),
        value => value,
    }
}

fn tool_name(method: &str, path: &str) -> String {
    let suffix = path
        .trim_start_matches("/api/v1/")
        .trim_matches('/')
        .replace('{', "by_")
        .replace('}', "")
        .replace('/', "_")
        .replace('-', "_");
    format!("enda_{method}_{suffix}")
}

/// MCP server that exposes every operation currently documented by My Enda.
pub struct EndaServer {
    api_client: ApiClient,
    catalog: ApiCatalog,
}

impl EndaServer {
    fn new(auth: crate::auth::AuthClient, catalog: ApiCatalog) -> Self {
        Self {
            api_client: ApiClient::new(auth),
            catalog,
        }
    }
}

impl ServerHandler for EndaServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "My Enda API tools are generated from the deployed OpenAPI contract. Supply request payloads in the typed `body` property.",
        )
    }

    fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, ErrorData>>
    + rmcp::service::MaybeSendFuture
    + '_ {
        async move {
            Ok(ListToolsResult {
                tools: self.catalog.tools(),
                ..Default::default()
            })
        }
    }

    fn get_tool(&self, name: &str) -> Option<Tool> {
        self.catalog
            .operations
            .get(name)
            .map(|operation| operation.tool.clone())
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, ErrorData>>
    + rmcp::service::MaybeSendFuture
    + '_ {
        async move {
            let Some(operation) = self.catalog.operations.get(request.name.as_ref()) else {
                return Err(ErrorData::invalid_params("Unknown My Enda tool", None));
            };
            let arguments = request.arguments.unwrap_or_default();
            match self.api_client.execute(operation, arguments).await {
                Ok(response) => Ok(CallToolResult::success(vec![ContentBlock::text(response)])),
                Err(error) => Ok(CallToolResult::error(vec![ContentBlock::text(
                    error.to_string(),
                )])),
            }
        }
    }
}

/// Downloads the API contract before serving MCP. This makes the tool inventory
/// and all request schemas track the deployed My Enda API, not a hand-maintained copy.
pub async fn start(auth: crate::auth::AuthClient) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let catalog = ApiCatalog::download(&config)
        .await
        .map_err(std::io::Error::other)?;
    eprintln!("Loaded {} My Enda API tools", catalog.operations.len());

    let service = EndaServer::new(auth, catalog).serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
