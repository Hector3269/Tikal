use crate::domain::repositories::executor::QueryExecutor;
use crate::domain::{error::TikalError, TikalResult};
use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::query_builder::generators::SqlGeneratorEnum;
use std::sync::Arc;

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create_pool(
        config: &DatabaseConfig,
    ) -> TikalResult<crate::infrastructure::database::DatabasePool> {
        match config.driver.as_str() {
            #[cfg(feature = "mysql")]
            "mysql" => Self::create_mysql_pool(config).await,

            #[cfg(feature = "postgres")]
            "postgres" => Self::create_postgres_pool(config).await,

            #[cfg(feature = "sqlite")]
            "sqlite" => Self::create_sqlite_pool(config).await,

            _ => Err(TikalError::config(&format!(
                "Unsupported database driver: {}. Available drivers: {}",
                config.driver,
                Self::available_drivers()
            ))),
        }
    }

    pub async fn create_executor(config: &DatabaseConfig) -> TikalResult<Arc<dyn QueryExecutor>> {
        match config.driver.as_str() {
            #[cfg(feature = "mysql")]
            "mysql" => Self::create_mysql_executor(config).await,

            #[cfg(feature = "postgres")]
            "postgres" => Self::create_postgres_executor(config).await,

            #[cfg(feature = "sqlite")]
            "sqlite" => Self::create_sqlite_executor(config).await,

            _ => Err(TikalError::config(&format!(
                "Unsupported database driver: {}. Available drivers: {}",
                config.driver,
                Self::available_drivers()
            ))),
        }
    }

    pub fn create_generator(driver: &str) -> TikalResult<SqlGeneratorEnum> {
        match driver {
            "mysql" => Ok(SqlGeneratorEnum::mysql()),
            "postgres" => Ok(SqlGeneratorEnum::postgres()),
            "sqlite" => Ok(SqlGeneratorEnum::sqlite()),
            _ => Err(TikalError::config(&format!(
                "Unsupported SQL generator for driver: {}",
                driver
            ))),
        }
    }

    pub async fn create_database(
        config: &DatabaseConfig,
    ) -> TikalResult<(Arc<dyn QueryExecutor>, SqlGeneratorEnum)> {
        let executor = Self::create_executor(config).await?;
        let generator = Self::create_generator(&config.driver)?;
        Ok((executor, generator))
    }

    fn available_drivers() -> String {
        let mut drivers = Vec::new();
        #[cfg(feature = "mysql")]
        drivers.push("mysql");
        #[cfg(feature = "postgres")]
        drivers.push("postgres");
        #[cfg(feature = "sqlite")]
        drivers.push("sqlite");

        if drivers.is_empty() {
            "none (enable features: mysql, postgres, or sqlite)".to_string()
        } else {
            drivers.join(", ")
        }
    }

    #[cfg(feature = "mysql")]
    async fn create_mysql_pool(
        config: &DatabaseConfig,
    ) -> TikalResult<crate::infrastructure::database::DatabasePool> {
        use crate::infrastructure::database::DatabasePool;

        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(Self::get_max_connections())
            .connect(&config.to_url())
            .await
            .map_err(|e| TikalError::config(&format!("Failed to connect to MySQL: {}", e)))?;

        Ok(DatabasePool::MySql(Arc::new(pool)))
    }

    #[cfg(feature = "mysql")]
    async fn create_mysql_executor(config: &DatabaseConfig) -> TikalResult<Arc<dyn QueryExecutor>> {
        use crate::infrastructure::drivers::mysql::MySqlExecutor;

        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(Self::get_max_connections())
            .connect(&config.to_url())
            .await
            .map_err(|e| TikalError::config(&format!("Failed to connect to MySQL: {}", e)))?;

        Ok(Arc::new(MySqlExecutor::new(pool)))
    }

    #[cfg(feature = "postgres")]
    async fn create_postgres_pool(
        config: &DatabaseConfig,
    ) -> TikalResult<crate::infrastructure::database::DatabasePool> {
        use crate::infrastructure::database::DatabasePool;

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(Self::get_max_connections())
            .connect(&config.to_url())
            .await
            .map_err(|e| TikalError::config(&format!("Failed to connect to PostgreSQL: {}", e)))?;

        Ok(DatabasePool::Postgres(Arc::new(pool)))
    }

    #[cfg(feature = "postgres")]
    async fn create_postgres_executor(
        config: &DatabaseConfig,
    ) -> TikalResult<Arc<dyn QueryExecutor>> {
        use crate::infrastructure::drivers::postgres::PostgresExecutor;

        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(Self::get_max_connections())
            .connect(&config.to_url())
            .await
            .map_err(|e| TikalError::config(&format!("Failed to connect to PostgreSQL: {}", e)))?;

        Ok(Arc::new(PostgresExecutor::new(pool)))
    }

    #[cfg(feature = "sqlite")]
    async fn create_sqlite_pool(
        config: &DatabaseConfig,
    ) -> TikalResult<crate::infrastructure::database::DatabasePool> {
        use crate::infrastructure::database::DatabasePool;
        use std::fs;
        use std::path::Path;

        let db_path = &config.database;
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| {
                    TikalError::config(&format!(
                        "Failed to create SQLite database directory: {}",
                        e
                    ))
                })?;
            }
        }

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1) // SQLite works best with single connection
            .connect(&config.to_url())
            .await
            .map_err(|e| TikalError::config(&format!("Failed to connect to SQLite: {}", e)))?;

        sqlx::query::<sqlx::Sqlite>("PRAGMA journal_mode = WAL")
            .execute(&pool)
            .await
            .ok();

        sqlx::query::<sqlx::Sqlite>("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .ok();

        Ok(DatabasePool::Sqlite(Arc::new(pool)))
    }

    #[cfg(feature = "sqlite")]
    async fn create_sqlite_executor(
        config: &DatabaseConfig,
    ) -> TikalResult<Arc<dyn QueryExecutor>> {
        use crate::infrastructure::drivers::sqlite::SqliteExecutor;
        use std::fs;
        use std::path::Path;

        let db_path = &config.database;
        if let Some(parent) = Path::new(db_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| {
                    TikalError::config(&format!(
                        "Failed to create SQLite database directory: {}",
                        e
                    ))
                })?;
            }
        }

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1) // SQLite works best with single connection
            .connect(&config.to_url())
            .await
            .map_err(|e| TikalError::config(&format!("Failed to connect to SQLite: {}", e)))?;

        sqlx::query::<sqlx::Sqlite>("PRAGMA journal_mode = WAL")
            .execute(&pool)
            .await
            .ok();

        sqlx::query::<sqlx::Sqlite>("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .ok();

        Ok(Arc::new(SqliteExecutor::new(pool)))
    }

    fn get_max_connections() -> u32 {
        std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10)
    }
}
