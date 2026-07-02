use serde::de::DeserializeOwned;
use crate::config::Config;

pub struct ApiClient {
    // Shared HTTP client used for all backend requests
    client: reqwest::Client,
    // Application config loaded from environment
    config: Config,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Config::load(),
        }
    }

// Sends a GET request to the backend, with deserialising the JSON response
pub async fn get_json<T>(
    &self,
    path: &str,

) -> Result<T, reqwest::Error>
where
T:DeserializeOwned,
{
    self.client
    .get(format!("{}{}", self.config.api_base_url, path))
    .send()
    .await?
    .json::<T>()
    .await
}
}