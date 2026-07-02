use serde::de::DeserializeOwned;
use crate::config::Config;

pub struct ApiClient {
    client: reqwest::Client,
    config: Config,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Config::load(),
        }
    }


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