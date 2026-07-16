// Stores application config loaded from the environment varaibles.
pub struct Config {
    pub api_base_url: String,
    pub openapi_url: String,
    pub keycloak_base: String,
    pub keycloak_realm: String,
    pub keycloak_id: String,
    pub redirect_uri: String,
}

impl Config {
    // Loads the needed config from the environment.
    pub fn load() -> Self {
        Self {
            api_base_url: std::env::var("ENDA_API_BASE_URL")
                .expect("ENDA_API_BASE_URL must be set"),

            openapi_url: std::env::var("ENDA_OPENAPI_URL")
                .unwrap_or_else(|_| "https://api.hederacourt.site/v3/api-docs".to_owned()),

            keycloak_base: std::env::var("ENDA_KEYCLOAK_BASE")
                .expect("ENDA_KEYCLOAK_BASE must be set"),

            keycloak_realm: std::env::var("ENDA_KEYCLOAK_REALM")
                .expect("ENDA_KEYCLOAK_REALM must be set"),

            keycloak_id: std::env::var("ENDA_KEYCLOAK_ID").expect("ENDA_KEYCLOAK_ID must be set"),

            redirect_uri: std::env::var("ENDA_REDIRECT_URI")
                .expect("ENDA_REDIRECT_URI must be set"),
        }
    }
}
