use serde::Deserialize;
use crate::config::Config;
use std::time::Instant;
use std::net::TcpListener;
use std::io::{Read, Write};
use oauth2::{
    PkceCodeChallenge,
    PkceCodeVerifier,
    AuthUrl,
    ClientId,
    CsrfToken,
    RedirectUrl,
    Scope,
    TokenUrl,
    basic::BasicClient,
};
#[derive(Debug)]
pub enum AuthError {
    OpenIdNotLoaded,
    PkceNotGenerated,
    BrowserError(String),
    Network(reqwest::Error),
}

impl From<reqwest::Error> for AuthError {
    fn from(error: reqwest::Error) -> Self{
        AuthError::Network(error)
    }
}
pub struct AuthClient{
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
    pub async fn discover(
        &mut self,
    ) -> Result<(), AuthError>{ 

        let url = format!(
            "{}/realms/{}/.well-known/openid-configuration",
            self.config.keycloak_base,
            self.config.keycloak_realm
        );

        let configuration = self.client
        .get(url)
        .send()
        .await?
        .json::<OpenIdConfiguration>()
        .await?;

        self.openid_config = Some(configuration);

        Ok(())
    }

    pub fn generate_pkce(
    &mut self,
){

    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

    self.pkce_challenge = Some(challenge);
    self.pkce_verifier = Some(verifier);
    self.state = Some(CsrfToken::new_random());


}

pub fn authorization_url(
    &self,
) -> Result<String, AuthError> {

    let openid = self
    .openid_config
    .as_ref()
    .ok_or(AuthError::OpenIdNotLoaded)?;

    let challenge = self
    .pkce_challenge
    .as_ref()
    .ok_or(AuthError::PkceNotGenerated)?;

    let url = format!(
    "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid&code_challenge={}&code_challenge_method=S256",
    openid.authorization_endpoint,
    self.config.keycloak_id,
    self.config.redirect_uri,
    challenge.as_str(),
);

    Ok(url)
}

pub fn open_browser(
    &self,
) -> Result<(), AuthError> {

    let url = self.authorization_url()?;

    eprintln!("Opening URL:");
    eprintln!("{}", url);

    webbrowser::open(&url)
        .map_err(|e| AuthError::BrowserError(e.to_string()))?;

    Ok(())
}

pub async fn login(
    &mut self,
) -> Result<(), AuthError>{

    self.discover().await?;
    self.generate_pkce();

    self.start_callback_listener()?;

    self.open_browser()?;

    self.wait_for_callback().await?;
    self.exchange_code().await?;

    Ok(())
}

pub async fn wait_for_callback(
    &mut self,
) -> Result<(), AuthError> {

    
    let listener = self
        .listener
        .as_ref()
        .expect("Callback listener not started");

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


    let path = request_line
    .split_whitespace()
    .nth(1)
    .unwrap_or("");

    let query = path
    .split('?')
    .nth(1)
    .unwrap_or("");


    for parameter in query.split('&') {

    if let Some(code) = parameter.strip_prefix("code=") {

        eprintln!("Authorization code:");
        eprintln!("{}", code);

        self.authorization_code = Some(code.to_string());
        eprintln!("Authorization code stored successfully!");
    }
}


    let response =
        "HTTP/1.1 200 OK\r\n\
         Content-Type: text/html\r\n\r\n\
         <html><body><h1>Login successful</h1>\
         You can close the window</body></html>";

    stream
        .write_all(response.as_bytes())
        .map_err(|e| AuthError::BrowserError(e.to_string()))?;


    Ok(())
}

pub async fn exchange_code(
    &mut self,
) -> Result<(), AuthError> {
    eprintln!("Exchanging authorization code for access token...");

    let code = self
        .authorization_code
        .as_ref()
        .expect("No authorization code");

    let verifier = self
        .pkce_verifier
        .as_ref()
        .expect("No PKCE verifier");

    let openid = self
        .openid_config
        .as_ref()
        .expect("OpenID config missing");

    let response = self.client
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

    let token = response
            .json::<TokenResponse>()
            .await?;

     self.access_token = Some(token.access_token);
     self.refresh_token = Some(token.refresh_token);
     
     self.expires_at = Some(
        Instant::now()
           // + std::time::Duration::from_secs(token.expires_in)
     );

     eprintln!("Tokens stored successfully");
     eprintln!("Expires in {} seconds", token.expires_in);
    Ok(())
}
pub fn start_callback_listener(
    &mut self,
) -> Result<(), AuthError> {

    let listener = TcpListener::bind("127.0.0.1:8000")
    .map_err(|e| AuthError::BrowserError(e.to_string()))?;

    eprintln! ("Listening on {}",
               listener.local_addr().unwrap());

    self.listener = Some(listener);

    Ok(())
}

pub async fn get_valid_token(
    &mut self,
) -> Result<&str, AuthError> {
    eprintln!("get_valid_token() called");
    let expires_at = self
        .expires_at
        .expect("No expiry time stored");

    if Instant::now() >= expires_at {

        eprintln!("Access token has expired.");

    }

    let token = self
    .access_token
    .as_deref()
    .expect("No access token available");

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


