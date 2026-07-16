use std::env;

pub struct Config {
    pub openrouter_key: String,
}

impl Config {

    pub fn load() -> Self {

        dotenvy::dotenv().ok();

        Self {
            openrouter_key: env::var("OPENROUTER_API_KEY")
                .expect("OPENROUTER_API_KEY must be set"),
        }
    }
}