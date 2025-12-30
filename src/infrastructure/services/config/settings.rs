use std::env;

use crate::domain::shared::db::DriverName;
use crate::infrastructure::services::config::database::DatabaseConfig;
use crate::infrastructure::services::config::logging::LoggingConfig;
use crate::Error;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

impl Settings {
    pub fn from_env() -> Result<Self, Error> {
        let driver = env_var_required("DATABASE_DRIVER")?;
        let driver = driver
            .parse()
            .map_err(|_| Error::config("Invalid DATABASE_DRIVER"))?;

        let database = match driver {
            DriverName::SQLite => {
                let path =
                    env::var("DATABASE_PATH").unwrap_or_else(|_| "database.sqlite".to_string());

                DatabaseConfig::sqlite(path)
            }

            DriverName::MySQL | DriverName::PostgreSQL => DatabaseConfig {
                driver,
                host: Some(env_var_required("DATABASE_HOST")?),
                port: Some(
                    env_var_required("DATABASE_PORT")?
                        .parse()
                        .map_err(|_| Error::config("DATABASE_PORT must be a number"))?,
                ),
                database: env_var_required("DATABASE_NAME")?,
                username: Some(env_var_required("DATABASE_USERNAME")?),
                password: Some(env::var("DATABASE_PASSWORD").unwrap_or_default()),
                ..Default::default()
            },
        };

        database.validate()?;

        let logging = LoggingConfig::from_env()?;

        Ok(Self { database, logging })
    }

    /// Create settings programmatically
    pub fn new(database: DatabaseConfig) -> Self {
        Self {
            database,
            logging: LoggingConfig::default(),
        }
    }

    /// Create settings programmatically with custom logging
    pub fn with_logging(database: DatabaseConfig, logging: LoggingConfig) -> Self {
        Self { database, logging }
    }
}

fn env_var_required(key: &str) -> Result<String, Error> {
    env::var(key).map_err(|_| Error::config(&format!("{} is required", key)))
}
