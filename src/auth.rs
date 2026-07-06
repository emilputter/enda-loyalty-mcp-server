use serde::Deserialize;
use crate::config::Config;

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

pub struct AuthClient{
    client: reqwest::Client,
    config: Config,
    openid_config: Option<OpenIdConfiguration>,
    pkce_challenge: Option<PkceCodeChallenge>,
    pkce_verifier: Option<PkceCodeVerifier>,
}

impl AuthClient {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
            client: reqwest::Client::new(),
            openid_config: None,
            pkce_challenge: None,
            pkce_verifier: None,
        }
    }
    pub async fn discover(
        &mut self,
    ) -> Result<(), reqwest::Error>{ 

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


}

pub fn authorization_url(
    &self,
) -> Result<String, String> {

    let openid = self
    .openid_config
    .as_ref()
    .ok_or("OpenID config not yet loaded")?;

    let challenge = self
    .pkce_challenge
    .as_ref()
    .ok_or("PKCE challenge yet to be generated")?;

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

pub fn open_browser(
    &self,
) -> Result<(), String> {

    let url = self.authorization_url()?;

    webbrowser::open(&url)
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn login(
    &mut self,
) -> Result<(), Box<dyn std::error::Error>>{

    self.discover().await?;
    self.generate_pkce();
    self.open_browser()?;

    Ok(())
}

}



#[derive(Debug, Deserialize)]
pub struct OpenIdConfiguration {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}


