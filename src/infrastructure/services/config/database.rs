use crate::domain::shared::db::DriverName;
use crate::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub driver: DriverName,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            driver: DriverName::SQLite,
            host: None,
            port: None,
            database: "database.db".to_string(),
            username: None,
            password: None,
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

    pub fn connection_url(&self) -> Result<String, Error> {
        self.validate()?;

        Ok(match self.driver {
            DriverName::SQLite => format!("sqlite:{}?mode=rwc", self.database),

            DriverName::MySQL => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username
                    .as_ref()
                    .expect("Username should be present after validation"),
                self.password.as_ref().unwrap_or(&"".to_string()),
                self.host
                    .as_ref()
                    .expect("Host should be present after validation"),
                self.port.expect("Port should be present after validation"),
                self.database
            ),

            DriverName::PostgreSQL => format!(
                "postgresql://{}:{}@{}:{}/{}",
                self.username
                    .as_ref()
                    .expect("Username should be present after validation"),
                self.password.as_ref().unwrap_or(&"".to_string()),
                self.host
                    .as_ref()
                    .expect("Host should be present after validation"),
                self.port.expect("Port should be present after validation"),
                self.database
            ),
        })
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.database.trim().is_empty() {
            return Err(Error::config("Database name cannot be empty"));
        }

        match self.driver {
            DriverName::SQLite => Ok(()),

            DriverName::MySQL | DriverName::PostgreSQL => {
                if self.host.as_deref().unwrap_or("").is_empty() {
                    return Err(Error::config("Host is required"));
                }
                if self.username.as_deref().unwrap_or("").is_empty() {
                    return Err(Error::config("Username is required"));
                }
                if self.port.is_none() || self.port.unwrap_or(0) == 0 {
                    return Err(Error::config("Port must be greater than 0"));
                }
                Ok(())
            }
        }
    }
}
