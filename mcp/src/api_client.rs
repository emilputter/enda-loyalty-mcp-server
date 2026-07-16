use crate::config::Config;
use crate::server::{ApiOperation, ApiParameter};
use reqwest::Method;
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    Authentication(crate::auth::AuthError),
    InvalidInput(String),
    Response { status: reqwest::StatusCode, body: String },
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<crate::auth::AuthError> for ApiError {
    fn from(error: crate::auth::AuthError) -> Self {
        Self::Authentication(error)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(error) => write!(f, "Backend request error: {error}"),
            Self::Authentication(error) => write!(f, "Authentication error: {error}"),
            Self::InvalidInput(error) => write!(f, "Invalid tool input: {error}"),
            Self::Response { status, body } => write!(f, "ENDA API returned {status}: {body}"),
        }
    }
}

impl std::error::Error for ApiError {}

pub struct ApiClient {
    client: reqwest::Client,
    config: Config,
    auth: Arc<Mutex<crate::auth::AuthClient>>,
}

impl ApiClient {
    pub fn new(auth: crate::auth::AuthClient) -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Config::load(),
            auth: Arc::new(Mutex::new(auth)),
        }
    }

    pub async fn execute(
        &self,
        operation: &ApiOperation,
        arguments: Map<String, Value>,
    ) -> Result<String, ApiError> {
        let token = {
            let mut auth = self.auth.lock().await;
            auth.get_valid_token().await?.to_string()
        };
        let method = Method::from_bytes(operation.method.as_bytes())
            .map_err(|error| ApiError::InvalidInput(format!("unsupported HTTP method: {error}")))?;
        let mut path = operation.path.clone();
        let mut query = Vec::new();
        let mut headers = Vec::new();

        for ApiParameter { name, location } in &operation.parameters {
            let Some(value) = arguments.get(name) else {
                continue;
            };
            match location.as_str() {
                "path" => {
                    let value = scalar(value, name)?;
                    path = path.replace(&format!("{{{name}}}"), &encode_path_segment(&value));
                }
                "query" => append_query(&mut query, name, value)?,
                "header" => headers.push((name.clone(), scalar(value, name)?)),
                _ => {}
            }
        }
        if path.contains('{') {
            return Err(ApiError::InvalidInput(format!(
                "missing a required path parameter for {path}"
            )));
        }

        let url = api_url(&self.config.api_base_url, &path);
        let mut request = self
            .client
            .request(method, url)
            .bearer_auth(token)
            .query(&query);
        for (name, value) in headers {
            request = request.header(name, value);
        }
        if let (Some(media_type), Some(body)) = (&operation.body_media_type, arguments.get("body"))
        {
            request = if media_type.starts_with("multipart/form-data") {
                request.multipart(multipart_form(body)?)
            } else {
                request.json(body)
            };
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            return Err(ApiError::Response { status, body: text });
        }
        match serde_json::from_str::<Value>(&text) {
            Ok(json) => serde_json::to_string_pretty(&json).map_err(|error| {
                ApiError::InvalidInput(format!("could not format JSON response: {error}"))
            }),
            Err(_) if text.is_empty() => Ok("{}".to_owned()),
            Err(_) => Ok(
                serde_json::to_string_pretty(&Value::String(text)).map_err(|error| {
                    ApiError::InvalidInput(format!("could not format response: {error}"))
                })?,
            ),
        }
    }
}

fn api_url(base_url: &str, documented_path: &str) -> String {
    let base = base_url.trim_end_matches('/');
    let path = if base.ends_with("/api/v1") {
        documented_path
            .strip_prefix("/api/v1")
            .unwrap_or(documented_path)
    } else {
        documented_path
    };
    format!("{base}{path}")
}

fn encode_path_segment(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                vec![byte as char]
            }
            byte => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

fn scalar(value: &Value, name: &str) -> Result<String, ApiError> {
    match value {
        Value::String(value) => Ok(value.clone()),
        Value::Number(value) => Ok(value.to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        _ => Err(ApiError::InvalidInput(format!(
            "{name} must be a scalar value"
        ))),
    }
}

fn append_query(
    query: &mut Vec<(String, String)>,
    name: &str,
    value: &Value,
) -> Result<(), ApiError> {
    match value {
        Value::Array(values) => {
            for value in values {
                query.push((name.to_owned(), scalar(value, name)?));
            }
        }
        Value::Object(values) => {
            for (key, value) in values {
                query.push((key.clone(), scalar(value, key)?));
            }
        }
        _ => query.push((name.to_owned(), scalar(value, name)?)),
    }
    Ok(())
}

fn multipart_form(body: &Value) -> Result<reqwest::multipart::Form, ApiError> {
    let fields = body
        .as_object()
        .ok_or_else(|| ApiError::InvalidInput("multipart body must be an object".to_owned()))?;
    let mut form = reqwest::multipart::Form::new();
    for (name, value) in fields {
        if value.is_null() {
            continue;
        }
        if name == "image" {
            if let Some((bytes, media_type)) = decode_data_url(scalar(value, name)?.as_str())? {
                let part = reqwest::multipart::Part::bytes(bytes)
                    .file_name("image")
                    .mime_str(&media_type)?;
                form = form.part(name.clone(), part);
                continue;
            }
        }
        form = form.text(name.clone(), scalar(value, name)?);
    }
    Ok(form)
}

/// MCP serializes inputs as JSON, so multipart file fields use a standard data
/// URL (`data:image/png;base64,...`) and are restored to binary before sending.
fn decode_data_url(value: &str) -> Result<Option<(Vec<u8>, String)>, ApiError> {
    let Some(value) = value.strip_prefix("data:") else {
        return Ok(None);
    };
    let (media_type, encoded) = value.split_once(",").ok_or_else(|| {
        ApiError::InvalidInput("image data URL is missing its payload".to_owned())
    })?;
    let media_type = media_type.strip_suffix(";base64").ok_or_else(|| {
        ApiError::InvalidInput("image data URL must be base64 encoded".to_owned())
    })?;
    if encoded.len() % 4 != 0 {
        return Err(ApiError::InvalidInput(
            "invalid base64 image length".to_owned(),
        ));
    }

    let mut bytes = Vec::with_capacity(encoded.len() / 4 * 3);
    for group in encoded.as_bytes().chunks_exact(4) {
        let padding = group.iter().filter(|&&byte| byte == b'=').count();
        let values = group
            .iter()
            .map(|&byte| base64_value(byte))
            .collect::<Result<Vec<_>, _>>()?;
        bytes.push((values[0] << 2) | (values[1] >> 4));
        if padding < 2 {
            bytes.push((values[1] << 4) | (values[2] >> 2));
        }
        if padding == 0 {
            bytes.push((values[2] << 6) | values[3]);
        }
    }
    Ok(Some((bytes, media_type.to_owned())))
}

fn base64_value(byte: u8) -> Result<u8, ApiError> {
    match byte {
        b'A'..=b'Z' => Ok(byte - b'A'),
        b'a'..=b'z' => Ok(byte - b'a' + 26),
        b'0'..=b'9' => Ok(byte - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        b'=' => Ok(0),
        _ => Err(ApiError::InvalidInput(
            "invalid base64 image payload".to_owned(),
        )),
    }
}
