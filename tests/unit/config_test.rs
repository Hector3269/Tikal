use tikal::infrastructure::{DatabaseConfig, LogFormat, LogLevel, LoggingConfig, Settings};
use tikal::kernel::types::db::DriverName;

#[test]
fn database_config_default() {
    let config = DatabaseConfig::default();
    assert_eq!(config.driver, DriverName::SQLite);
    assert_eq!(config.database, "database.sqlite");
    assert_eq!(config.max_connections, 10);
    assert_eq!(config.min_connections, 1);
    assert_eq!(config.connection_timeout_seconds, 30);
    assert_eq!(config.command_timeout_seconds, 30);
}

#[test]
fn database_config_sqlite() {
    let config = DatabaseConfig::sqlite("test.db");
    assert_eq!(config.driver, DriverName::SQLite);
    assert_eq!(config.database, "test.db");
    assert_eq!(config.host, None);
    assert_eq!(config.port, None);
    assert_eq!(config.username, None);
    assert_eq!(config.password, None);
}

#[test]
fn database_config_mysql() {
    let config = DatabaseConfig::mysql("localhost", 3306, "testdb", "user", "pass");
    assert_eq!(config.driver, DriverName::MySQL);
    assert_eq!(config.host, Some("localhost".to_string()));
    assert_eq!(config.port, Some(3306));
    assert_eq!(config.database, "testdb");
    assert_eq!(config.username, Some("user".to_string()));
    assert_eq!(config.password, Some("pass".to_string()));
}

#[test]
fn database_config_postgres() {
    let config = DatabaseConfig::postgres("localhost", 5432, "testdb", "user", "pass");
    assert_eq!(config.driver, DriverName::PostgreSQL);
    assert_eq!(config.host, Some("localhost".to_string()));
    assert_eq!(config.port, Some(5432));
    assert_eq!(config.database, "testdb");
    assert_eq!(config.username, Some("user".to_string()));
    assert_eq!(config.password, Some("pass".to_string()));
}

#[test]
fn database_config_connection_url_sqlite() {
    let config = DatabaseConfig::sqlite("test.db");
    assert_eq!(config.connection_url().unwrap(), "sqlite:test.db");
}

#[test]
fn database_config_connection_url_mysql() {
    let config = DatabaseConfig::mysql("localhost", 3306, "testdb", "user", "pass");
    assert_eq!(
        config.connection_url().unwrap(),
        "mysql://user:pass@localhost:3306/testdb"
    );
}

#[test]
fn database_config_connection_url_mysql_no_password() {
    let mut config = DatabaseConfig::mysql("localhost", 3306, "testdb", "user", "");
    config.password = None;
    assert_eq!(
        config.connection_url().unwrap(),
        "mysql://user:@localhost:3306/testdb"
    );
}

#[test]
fn database_config_connection_url_postgres() {
    let config = DatabaseConfig::postgres("localhost", 5432, "testdb", "user", "pass");
    assert_eq!(
        config.connection_url().unwrap(),
        "postgresql://user:pass@localhost:5432/testdb"
    );
}

#[test]
fn database_config_validate_sqlite_valid() {
    let config = DatabaseConfig::sqlite("test.db");
    assert!(config.validate().is_ok());
}

#[test]
fn database_config_validate_sqlite_empty_database() {
    let config = DatabaseConfig::sqlite("");
    assert!(config.validate().is_err());
}

#[test]
fn database_config_validate_mysql_valid() {
    let config = DatabaseConfig::mysql("localhost", 3306, "testdb", "user", "pass");
    assert!(config.validate().is_ok());
}

#[test]
fn database_config_validate_mysql_missing_host() {
    let mut config = DatabaseConfig::mysql("", 3306, "testdb", "user", "pass");
    config.host = Some("".to_string());
    assert!(config.validate().is_err());
}

#[test]
fn database_config_validate_mysql_missing_username() {
    let mut config = DatabaseConfig::mysql("localhost", 3306, "testdb", "", "pass");
    config.username = Some("".to_string());
    assert!(config.validate().is_err());
}

#[test]
fn database_config_validate_mysql_invalid_port() {
    let mut config = DatabaseConfig::mysql("localhost", 0, "testdb", "user", "pass");
    config.port = Some(0);
    assert!(config.validate().is_err());
}

#[test]
fn database_config_validate_postgres_valid() {
    let config = DatabaseConfig::postgres("localhost", 5432, "testdb", "user", "pass");
    assert!(config.validate().is_ok());
}

// Tests for settings.rs
#[test]
fn settings_from_env_sqlite() {
    // Set up environment variables
    unsafe {
        std::env::set_var("DATABASE_DRIVER", "sqlite");
        std::env::set_var("DATABASE_PATH", "test.db");
    }

    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.database.driver, DriverName::SQLite);
    assert_eq!(settings.database.database, "test.db");

    // Clean up
    unsafe {
        std::env::remove_var("DATABASE_DRIVER");
        std::env::remove_var("DATABASE_PATH");
    }
}

#[test]
fn settings_from_env_sqlite_default_path() {
    unsafe {
        std::env::set_var("DATABASE_DRIVER", "sqlite");
    }
    // Don't set DATABASE_PATH

    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.database.driver, DriverName::SQLite);
    assert_eq!(settings.database.database, "database.sqlite");
}

#[test]
fn settings_from_env_mysql() {
    unsafe {
        std::env::set_var("DATABASE_DRIVER", "mysql");
        std::env::set_var("DATABASE_HOST", "localhost");
        std::env::set_var("DATABASE_PORT", "3306");
        std::env::set_var("DATABASE_NAME", "testdb");
        std::env::set_var("DATABASE_USERNAME", "user");
        std::env::set_var("DATABASE_PASSWORD", "pass");
    }

    let settings = Settings::from_env().unwrap();
    assert_eq!(settings.database.driver, DriverName::MySQL);
    assert_eq!(settings.database.host, Some("localhost".to_string()));
    assert_eq!(settings.database.port, Some(3306));
    assert_eq!(settings.database.database, "testdb");
    assert_eq!(settings.database.username, Some("user".to_string()));
    assert_eq!(settings.database.password, Some("pass".to_string()));

    // Clean up
    unsafe {
        std::env::remove_var("DATABASE_DRIVER");
        std::env::remove_var("DATABASE_HOST");
        std::env::remove_var("DATABASE_PORT");
        std::env::remove_var("DATABASE_NAME");
        std::env::remove_var("DATABASE_USERNAME");
        std::env::remove_var("DATABASE_PASSWORD");
    }
}

#[test]
fn settings_from_env_missing_required_env_var() {
    // Don't set any environment variables
    let result = Settings::from_env();
    assert!(result.is_err());
}

#[test]
fn settings_from_env_invalid_driver() {
    unsafe {
        std::env::set_var("DATABASE_DRIVER", "invalid");
    }

    let result = Settings::from_env();
    assert!(result.is_err());

    unsafe {
        std::env::remove_var("DATABASE_DRIVER");
    }
}

#[test]
fn settings_from_env_mysql_missing_host() {
    unsafe {
        std::env::set_var("DATABASE_DRIVER", "mysql");
        std::env::set_var("DATABASE_PORT", "3306");
        std::env::set_var("DATABASE_NAME", "testdb");
        std::env::set_var("DATABASE_USERNAME", "user");
        std::env::remove_var("DATABASE_HOST"); // Explicitly remove it
    }
    // Missing DATABASE_HOST

    let result = Settings::from_env();
    assert!(result.is_err());

    // Clean up
    unsafe {
        std::env::remove_var("DATABASE_DRIVER");
        std::env::remove_var("DATABASE_PORT");
        std::env::remove_var("DATABASE_NAME");
        std::env::remove_var("DATABASE_USERNAME");
        std::env::remove_var("DATABASE_HOST");
    }
}

#[test]
fn settings_from_env_mysql_invalid_port() {
    unsafe {
        std::env::set_var("DATABASE_DRIVER", "mysql");
        std::env::set_var("DATABASE_HOST", "localhost");
        std::env::set_var("DATABASE_PORT", "invalid");
        std::env::set_var("DATABASE_NAME", "testdb");
        std::env::set_var("DATABASE_USERNAME", "user");
    }

    let result = Settings::from_env();
    assert!(result.is_err());

    // Clean up
    unsafe {
        std::env::remove_var("DATABASE_DRIVER");
        std::env::remove_var("DATABASE_HOST");
        std::env::remove_var("DATABASE_PORT");
        std::env::remove_var("DATABASE_NAME");
        std::env::remove_var("DATABASE_USERNAME");
    }
}

// Tests for logging.rs
#[test]
fn log_level_default() {
    assert_eq!(LogLevel::default(), LogLevel::Info);
}

#[test]
fn logging_config_default() {
    let config = LoggingConfig::default();
    assert_eq!(config.level, LogLevel::Info);
    assert_eq!(config.format, LogFormat::Json);
}
