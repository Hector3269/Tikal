use crate::domain::{TikalError, TikalResult};
use std::env;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    pub fn from_env() -> TikalResult<Self> {
        dotenvy::dotenv().ok();

        let driver = env::var("DATABASE_DRIVER")
            .map_err(|_| TikalError::config("DATABASE_DRIVER environment variable not set"))?;
        let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DATABASE_PORT")
            .unwrap_or_else(|_| "3306".to_string())
            .parse()
            .map_err(|_| TikalError::config("DATABASE_PORT must be a valid port number"))?;
        let database = env::var("DATABASE_NAME")
            .map_err(|_| TikalError::config("DATABASE_NAME environment variable not set"))?;

        let username = if driver == "sqlite" {
            "sqlite".to_string() // dummy value
        } else {
            env::var("DATABASE_USERNAME")
                .map_err(|_| TikalError::config("DATABASE_USERNAME environment variable not set"))?
        };

        let password = if driver == "sqlite" {
            "".to_string() // dummy value
        } else {
            env::var("DATABASE_PASSWORD")
                .map_err(|_| TikalError::config("DATABASE_PASSWORD environment variable not set"))?
        };

        Ok(DatabaseConfig {
            driver,
            host,
            port,
            database,
            username,
            password,
        })
    }

    pub fn to_url(&self) -> String {
        match self.driver.as_str() {
            "mysql" => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            "postgres" => format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            "sqlite" => format!("sqlite:{}", self.database),
            _ => panic!("Unsupported database driver: {}", self.driver),
        }
    }
}
