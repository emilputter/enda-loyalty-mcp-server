use crate::config::Config;
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Instant;
#[derive(Debug)]
pub enum AuthError {
    OpenIdNotLoaded,
    PkceNotGenerated,
    BrowserError(String),
    Network(reqwest::Error),

    ListenerNotStarted,
    AuthorizationCodeMissing,
    PkceVerifierMissing,
    RefreshTokenMissing,
    ExpiryMissing,
    AccessTokenMissing,
}

// For errors that could occur during the authentication process
impl From<reqwest::Error> for AuthError {
    fn from(error: reqwest::Error) -> Self {
        AuthError::Network(error)
    }
}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::OpenIdNotLoaded => {
                write!(f, "OpenID configuration has not been loaded")
            }
            AuthError::PkceNotGenerated => {
                write!(f, "PKCE challenge has not been generated")
            }
            AuthError::BrowserError(error) => {
                write!(f, "Browser error: {}", error)
            }
            AuthError::Network(error) => {
                write!(f, "Network error: {}", error)
            }

            AuthError::ListenerNotStarted => {
                write!(f, "Callback listener has not been started")
            }
            AuthError::AuthorizationCodeMissing => {
                write!(f, "Authorization code is missing")
            }
            AuthError::PkceVerifierMissing => {
                write!(f, "PKCE verifier is missing")
            }
            AuthError::RefreshTokenMissing => {
                write!(f, "Refresh token is missing")
            }
            AuthError::ExpiryMissing => {
                write!(f, "Token expiry time is missing")
            }
            AuthError::AccessTokenMissing => {
                write!(f, "Access token is missing")
            }
        }
    }
}
impl std::error::Error for AuthError {}
//Handles OAuth authentication with keycloak, including login, token management, and automatic token refresh
pub struct AuthClient {
    client: reqwest::Client,
    config: Config,
    openid_config: Option<OpenIdConfiguration>,
    pkce_challenge: Option<PkceCodeChallenge>,
    pkce_verifier: Option<PkceCodeVerifier>,
    state: Option<CsrfToken>,

    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<Instant>,
    authorization_code: Option<String>,

    listener: Option<TcpListener>,
}

impl AuthClient {
    // creates a new authentication client with no active user session
    pub fn new() -> Self {
        Self {
            config: Config::load(),
            client: reqwest::Client::new(),
            openid_config: None,
            pkce_challenge: None,
            pkce_verifier: None,
            access_token: None,
            refresh_token: None,
            expires_at: None,
            authorization_code: None,
            state: None,
            listener: None,
        }
    }
    //Retrieves the openID config from keycloak, so that authorization and token endpoints can be discovered dynamically
    pub async fn discover(&mut self) -> Result<(), AuthError> {
        let url = format!(
            "{}/realms/{}/.well-known/openid-configuration",
            self.config.keycloak_base, self.config.keycloak_realm
        );

        let configuration = self
            .client
            .get(url)
            .send()
            .await?
            .json::<OpenIdConfiguration>()
            .await?;
        self.openid_config = Some(configuration);

        Ok(())
    }

    //Generate a new PKCE challenge, verifier and CSRF state
    pub fn generate_pkce(&mut self) {
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

        self.pkce_challenge = Some(challenge);
        self.pkce_verifier = Some(verifier);
        self.state = Some(CsrfToken::new_random());
    }

    //Builds the keycloack authorization URL used to start the login flow
    pub fn authorization_url(&self) -> Result<String, AuthError> {
        let openid = self
            .openid_config
            .as_ref()
            .ok_or(AuthError::OpenIdNotLoaded)?;

        let challenge = self
            .pkce_challenge
            .as_ref()
            .ok_or(AuthError::PkceNotGenerated)?;

        let url = format!(
            "{}\
   ?client_id={}\
   &redirect_uri={}\
   &response_type=code\
   &scope=openid\
   &code_challenge={}\
   &code_challenge_method=S256",
            openid.authorization_endpoint,
            self.config.keycloak_id,
            self.config.redirect_uri,
            challenge.as_str(),
        );

        Ok(url)
    }

    // Opens the users default browser and navigates the keycloakc authorization page
    pub fn open_browser(&self) -> Result<(), AuthError> {
        let url = self.authorization_url()?;

        eprintln!("Opening browser: {}", url);

        webbrowser::open(&url).map_err(|e| AuthError::BrowserError(e.to_string()))?;

        Ok(())
    }

    // Executes the complete OAuth login flow and stores the resulting access and refresh tokens
    pub async fn login(&mut self) -> Result<(), AuthError> {
        self.discover().await?;
        self.generate_pkce();

        self.start_callback_listener()?;

        self.open_browser()?;

        self.wait_for_callback().await?;
        self.exchange_code().await?;

        Ok(())
    }

    // Waits for the OAuth redirect and receives the authorization code returned by Keycloak
    pub async fn wait_for_callback(&mut self) -> Result<(), AuthError> {
        let listener = self
            .listener
            .as_ref()
            .ok_or(AuthError::ListenerNotStarted)?;

        eprintln!("Waiting for callback...");

        let (mut stream, address) = listener
            .accept()
            .map_err(|e| AuthError::BrowserError(e.to_string()))?;

        eprintln!("Connection received from {}", address);

        let mut buffer = [0u8; 4096];

        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|e| AuthError::BrowserError(e.to_string()))?;

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        let request_line = request.lines().next().unwrap_or("");

        let path = request_line.split_whitespace().nth(1).unwrap_or("");

        let query = path.split('?').nth(1).unwrap_or("");

        for parameter in query.split('&') {
            if let Some(code) = parameter.strip_prefix("code=") {
                eprintln!("Authorization code recieved.");

                self.authorization_code = Some(code.to_string());
                eprintln!("Authorization code stored successfully!");
            }
        }
        let response = "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\r\n\
         <html><body><h1>Login successful</h1>\
         You can close the window</body></html>";

        stream
            .write_all(response.as_bytes())
            .map_err(|e| AuthError::BrowserError(e.to_string()))?;

        self.listener = None;
        Ok(())
    }

    // Exchanges the authorization code for an access token and refresh token
    pub async fn exchange_code(&mut self) -> Result<(), AuthError> {
        eprintln!("Exchanging authorization code for access token...");

        let code = self
            .authorization_code
            .as_ref()
            .ok_or(AuthError::AuthorizationCodeMissing)?;

        let verifier = self
            .pkce_verifier
            .as_ref()
            .ok_or(AuthError::PkceVerifierMissing)?;

        let openid = self
            .openid_config
            .as_ref()
            .ok_or(AuthError::OpenIdNotLoaded)?;

        let response = self
            .client
            .post(&openid.token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", &self.config.keycloak_id),
                ("code", code),
                ("redirect_uri", &self.config.redirect_uri),
                ("code_verifier", verifier.secret()),
            ])
            .send()
            .await?;

        eprintln!("Status: {}", response.status());

        let token = response.json::<TokenResponse>().await?;

        self.store_tokens(token);
        Ok(())
    }

    // Stores the latest OAuth tokens and calculates when the current access token expires.
    fn store_tokens(&mut self, token: TokenResponse) {
        self.access_token = Some(token.access_token);

        self.refresh_token = Some(token.refresh_token);

        self.expires_at = Some(Instant::now() + std::time::Duration::from_secs(token.expires_in));

        eprintln!("Tokens stored successfully");
        eprintln!("Expires in {} seconds", token.expires_in);
    }

    //Requests a new access token using the stored refresh token.
    pub async fn refresh_access_token(&mut self) -> Result<(), AuthError> {
        eprintln!("Refreshing access token");

        let refresh_token = self
            .refresh_token
            .as_ref()
            .ok_or(AuthError::RefreshTokenMissing)?;

        let openid = self
            .openid_config
            .as_ref()
            .ok_or(AuthError::OpenIdNotLoaded)?;
        let response = self
            .client
            .post(&openid.token_endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!(
                "grant_type=refresh_token&client_id={}&refresh_token={}",
                self.config.keycloak_id, refresh_token,
            ))
            .send()
            .await?;

        eprintln!("Refresh status: {}", response.status());

        let token = response.json::<TokenResponse>().await?;
        self.store_tokens(token);
        Ok(())
    }
    //Starts a local HTTP listener to recieve the OAuth callback from Keycloak
    pub fn start_callback_listener(&mut self) -> Result<(), AuthError> {
        let listener = TcpListener::bind("127.0.0.1:8000")
            .map_err(|e| AuthError::BrowserError(e.to_string()))?;

        eprintln!(
            "Listening on {}",
            listener
                .local_addr()
                .map_err(|e| AuthError::BrowserError(e.to_string()))?
        );

        self.listener = Some(listener);

        Ok(())
    }

    // Returns a valid access token, automatically being able to refresh it
    pub async fn get_valid_token(&mut self) -> Result<&str, AuthError> {
        let expires_at = self.expires_at.ok_or(AuthError::ExpiryMissing)?;

        if Instant::now() >= expires_at {
            eprintln!("Access token expired. Refreshing...");

            if let Err(error) = self.refresh_access_token().await {
                eprintln!("Refresh failed: {}", error);
                eprintln!("Starting login flow...");
                self.login().await?;
            }
        }
        let token = self
            .access_token
            .as_deref()
            .ok_or(AuthError::AccessTokenMissing)?;

        Ok(token)
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenIdConfiguration {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: Option<String>,
}
