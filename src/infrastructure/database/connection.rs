use crate::config::DatabaseConfig;
use crate::domain::error::{TikalError, TikalResult};
use crate::infrastructure::database::pool::DatabasePool;
use sqlx::{MySqlPool, PgPool, SqlitePool};
use std::fs;
use std::path::Path;

pub async fn create_pool(config: &DatabaseConfig) -> TikalResult<DatabasePool> {
    match config.driver.as_str() {
        "mysql" => {
            let pool = MySqlPool::connect(&config.to_url()).await?;
            Ok(DatabasePool::MySql(pool))
        }
        "postgres" => {
            let pool = PgPool::connect(&config.to_url()).await?;
            Ok(DatabasePool::Postgres(pool))
        }
        "sqlite" => {
            let db_path = &config.database;
            if let Some(parent) = Path::new(db_path).parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    TikalError::config(&format!("Failed to create database directory: {}", e))
                })?;
            }
            let pool = SqlitePool::connect(&config.to_url()).await?;
            Ok(DatabasePool::Sqlite(pool))
        }
        _ => Err(TikalError::config("Unsupported database driver")),
    }
}
