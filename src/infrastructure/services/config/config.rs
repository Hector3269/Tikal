use crate::Error;
use once_cell::sync::OnceCell;

use super::settings::Settings;

static SETTINGS: OnceCell<Settings> = OnceCell::new();

pub fn init() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    let settings = Settings::from_env()?;
    SETTINGS
        .set(settings)
        .map_err(|_| Error::config("Configuration already initialized"))?;
    Ok(())
}

pub fn init_with_settings(settings: Settings) -> Result<(), Error> {
    SETTINGS
        .set(settings)
        .map_err(|_| Error::config("Configuration already initialized"))?;
    Ok(())
}

pub fn database(
) -> Result<&'static crate::infrastructure::services::config::database::DatabaseConfig, Error> {
    Ok(&settings()?.database)
}

pub fn logging(
) -> Result<&'static crate::infrastructure::services::config::logging::LoggingConfig, Error> {
    Ok(&settings()?.logging)
}

pub fn settings() -> Result<&'static Settings, Error> {
    SETTINGS
        .get()
        .ok_or_else(|| Error::config("Configuration not initialized. Call config::init() first."))
}

pub fn setup_dev_defaults() -> Result<(), Error> {
    use std::fs;
    use std::path::Path;

    let env_path = Path::new(".env");

    if !env_path.exists() {
        let default_config = r#"# Database Configuration
DATABASE_DRIVER=sqlite
DATABASE_PATH=database.sqlite

# Logging Configuration
LOG_LEVEL=info
LOG_FORMAT=pretty
"#;

        fs::write(env_path, default_config)
            .map_err(|e| Error::config(&format!("Failed to create .env file: {}", e)))?;
    }

    Ok(())
}

pub struct Config;

impl Config {
    pub fn init() -> Result<(), Error> {
        init()
    }

    pub fn init_with_settings(settings: Settings) -> Result<(), Error> {
        init_with_settings(settings)
    }

    pub fn database(
    ) -> Result<&'static crate::infrastructure::services::config::database::DatabaseConfig, Error>
    {
        database()
    }

    pub fn logging(
    ) -> Result<&'static crate::infrastructure::services::config::logging::LoggingConfig, Error>
    {
        logging()
    }

    pub fn settings() -> Result<&'static Settings, Error> {
        settings()
    }

    pub fn setup_dev_defaults() -> Result<(), Error> {
        setup_dev_defaults()
    }
}
