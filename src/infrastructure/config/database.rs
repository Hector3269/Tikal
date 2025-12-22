use crate::kernel::error::KernelError;
use crate::kernel::types::db::DriverName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: DriverName,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,

    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout_seconds: u64,
    pub command_timeout_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            driver: DriverName::SQLite,
            host: None,
            port: None,
            database: "database.sqlite".to_string(),
            username: None,
            password: None,
            max_connections: 10,
            min_connections: 1,
            connection_timeout_seconds: 30,
            command_timeout_seconds: 30,
        }
    }
}

impl DatabaseConfig {
    pub fn sqlite(path: impl Into<String>) -> Self {
        Self {
            driver: DriverName::SQLite,
            database: path.into(),
            ..Default::default()
        }
    }

    pub fn mysql(
        host: impl Into<String>,
        port: u16,
        database: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            driver: DriverName::MySQL,
            host: Some(host.into()),
            port: Some(port),
            database: database.into(),
            username: Some(username.into()),
            password: Some(password.into()),
            ..Default::default()
        }
    }

    pub fn postgres(
        host: impl Into<String>,
        port: u16,
        database: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            driver: DriverName::PostgreSQL,
            host: Some(host.into()),
            port: Some(port),
            database: database.into(),
            username: Some(username.into()),
            password: Some(password.into()),
            ..Default::default()
        }
    }

    pub fn connection_url(&self) -> Result<String, KernelError> {
        self.validate()?;

        Ok(match self.driver {
            DriverName::SQLite => format!("sqlite:{}", self.database),

            DriverName::MySQL => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username.as_ref().unwrap(),
                self.password.as_ref().unwrap_or(&"".to_string()),
                self.host.as_ref().unwrap(),
                self.port.unwrap(),
                self.database
            ),

            DriverName::PostgreSQL => format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username.as_ref().unwrap(),
                self.password.as_ref().unwrap_or(&"".to_string()),
                self.host.as_ref().unwrap(),
                self.port.unwrap(),
                self.database
            ),
        })
    }

    pub fn validate(&self) -> Result<(), KernelError> {
        if self.database.trim().is_empty() {
            return Err(KernelError::config("Database name cannot be empty"));
        }

        match self.driver {
            DriverName::SQLite => Ok(()),

            DriverName::MySQL | DriverName::PostgreSQL => {
                if self.host.as_deref().unwrap_or("").is_empty() {
                    return Err(KernelError::config("Host is required"));
                }
                if self.username.as_deref().unwrap_or("").is_empty() {
                    return Err(KernelError::config("Username is required"));
                }
                if self.port.unwrap_or(0) == 0 {
                    return Err(KernelError::config("Port must be greater than 0"));
                }
                Ok(())
            }
        }
    }
}
