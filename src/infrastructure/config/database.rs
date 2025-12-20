use serde::{Deserialize, Serialize};
use crate::kernel::types::db::DriverName;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: DriverName,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connection_timeout_seconds: Option<u64>,
    pub command_timeout_seconds: Option<u64>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            driver: DriverName::SQLite,
            host: "localhost".to_string(),
            port: 3306,
            database: "tikal".to_string(),
            username: "root".to_string(),
            password: "".to_string(),
            max_connections: Some(10),
            min_connections: Some(1),
            connection_timeout_seconds: Some(30),
            command_timeout_seconds: Some(30),
        }
    }
}

impl DatabaseConfig {
    pub fn new(driver: DriverName, host: &str, port: u16, database: &str, username: &str, password: &str) -> Self {
        Self {
            driver,
            host: host.to_string(),
            port,
            database: database.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            ..Default::default()
        }
    }

    pub fn sqlite(database_path: &str) -> Self {
        Self {
            driver: DriverName::SQLite,
            database: database_path.to_string(),
            ..Default::default()
        }
    }

    pub fn mysql(host: &str, port: u16, database: &str, username: &str, password: &str) -> Self {
        Self::new(DriverName::MySQL, host, port, database, username, password)
    }

    pub fn postgres(host: &str, port: u16, database: &str, username: &str, password: &str) -> Self {
        Self::new(DriverName::PostgreSQL, host, port, database, username, password)
    }

    pub fn connection_url(&self) -> String {
        match self.driver {
            DriverName::SQLite => format!("sqlite:{}", self.database),
            DriverName::MySQL => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            DriverName::PostgreSQL => format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.database.trim().is_empty() {
            return Err("Database name cannot be empty".to_string());
        }

        match self.driver {
            DriverName::SQLite => {
            }
            DriverName::MySQL | DriverName::PostgreSQL => {
                if self.host.trim().is_empty() {
                    return Err("Host cannot be empty".to_string());
                }
                if self.username.trim().is_empty() {
                    return Err("Username cannot be empty".to_string());
                }
                if self.port == 0 {
                    return Err("Port must be greater than 0".to_string());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: Some(4),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self::default())
    }
    pub fn from_file(_path: &str) -> Result<Self, String> {
        Ok(Self::default())
    }
}