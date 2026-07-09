use crate::config::Config;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ApiClient {
    // Shared HTTP client used for all backend requests
    client: reqwest::Client,
    // Application config loaded from environment
    config: Config,
    //Handles the OAuth authentication and token management
    auth: Arc<Mutex<crate::auth::AuthClient>>,
}

impl ApiClient {
    //Creates a new API client using a provided session
    pub fn new(auth: crate::auth::AuthClient) -> Self {
        Self {
            client: reqwest::Client::new(),
            config: Config::load(),
            auth: Arc::new(Mutex::new(auth)),
        }
    }

    // Sends a GET request to the backend with deserialising the JSON response
    pub async fn get_json<T>(&self, path: &str) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let token = {
            let mut auth = self.auth.lock().await;
            auth.get_valid_token()
                .await
                .expect("Authentication failed")
                .to_string()
        };

        self.client
            .get(format!("{}{}", self.config.api_base_url, path))
            .bearer_auth(token)
            .send()
            .await?
            .json::<T>()
            .await
    }
}
