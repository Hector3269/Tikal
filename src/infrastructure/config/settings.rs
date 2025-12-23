use std::env;

use crate::infrastructure::config::database::DatabaseConfig;
use crate::infrastructure::config::logging::LoggingConfig;
use crate::kernel::error::KernelError;
use crate::kernel::types::db::DriverName;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

impl Settings {
    pub fn from_env() -> Result<Self, KernelError> {
        let driver = env_var_required("DATABASE_DRIVER")?;
        let driver = driver
            .parse()
            .map_err(|_| KernelError::config("Invalid DATABASE_DRIVER"))?;

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
                        .map_err(|_| KernelError::config("DATABASE_PORT must be a number"))?,
                ),
                database: env_var_required("DATABASE_NAME")?,
                username: Some(env_var_required("DATABASE_USERNAME")?),
                password: Some(env::var("DATABASE_PASSWORD").unwrap_or_default()),
                ..Default::default()
            },
        };

        database.validate()?;

        let logging = LoggingConfig::default();

        Ok(Self { database, logging })
    }
}

fn env_var_required(key: &str) -> Result<String, KernelError> {
    env::var(key).map_err(|_| KernelError::config(&format!("{} is required", key)))
}
