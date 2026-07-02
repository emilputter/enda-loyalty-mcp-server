// Stores application config loaded from the environment varaibles.
pub struct Config {
    pub api_base_url: String,
}

impl Config {
    // Loads the needed config from the environment.
    pub fn load() -> Self {
        Self{
            api_base_url: std::env::var("ENDA_API_BASE_URL")
            .expect("ENDA_API_BASE_URL must be set"),
        }
    }
}