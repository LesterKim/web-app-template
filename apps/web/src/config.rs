use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub addr: String,
    pub database_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let addr = env::var("WEB_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
        let database_url = env::var("DATABASE_URL").ok();

        Self { addr, database_url }
    }
}
