use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub postgres_url: String,
    pub mongo_url: String
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();

        let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set.");

        AppConfig {
            postgres_url,
            mongo_url,
        }
    }
}