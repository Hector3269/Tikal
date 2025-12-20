use async_trait::async_trait;
use super::DatabaseDriver;
use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;
use crate::infrastructure::types::DbResult;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SQLiteDriver {
    pool: Arc<Mutex<Option<SqlitePool>>>,
}

impl SQLiteDriver {
    pub fn new() -> Self {
        Self {
            pool: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect_with_url(&self, url: &str) -> DbResult<()> {
        let pool = SqlitePool::connect(url).await
            .map_err(|e| KernelError::connection("sqlite", &e.to_string()))?;
        *self.pool.lock().await = Some(pool);
        Ok(())
    }
}

impl Default for SQLiteDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DatabaseDriver for SQLiteDriver {
    async fn connect(&self) -> DbResult<()> {
        // Connection should be established via connect_with_url
        Ok(())
    }

    async fn execute(&self, sql: &str, params: &[Value]) -> DbResult<()> {
        let pool = self.pool.lock().await;
        if let Some(pool) = pool.as_ref() {
            let mut query = sqlx::query(sql);
            for param in params {
                query = match param {
                    Value::String(s) => query.bind(s),
                    Value::Int(i) => query.bind(*i),
                    Value::Bool(b) => query.bind(*b),
                    Value::Float(f) => query.bind(*f),
                    Value::Null => query.bind(None::<String>),
                };
            }
            query.execute(pool).await
                .map_err(|e| KernelError::query(sql, &e.to_string()))?;
        }
        Ok(())
    }

    async fn query(&self, sql: &str, params: &[Value]) -> DbResult<Vec<DbRow>> {
        let pool = self.pool.lock().await;
        if let Some(pool) = pool.as_ref() {
            let mut query = sqlx::query(sql);
            for param in params {
                query = match param {
                    Value::String(s) => query.bind(s),
                    Value::Int(i) => query.bind(*i),
                    Value::Bool(b) => query.bind(*b),
                    Value::Float(f) => query.bind(*f),
                    Value::Null => query.bind(None::<String>),
                };
            }

            let _rows = query.fetch_all(pool).await
                .map_err(|e| KernelError::query(sql, &e.to_string()))?;

            // For now, return empty results - full implementation would require
            // more complex column mapping. This is a placeholder.
            let results = Vec::new();

            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }

    async fn transaction<F, Fut>(&self, _f: F) -> DbResult<()>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = DbResult<()>> + Send,
    {
        let pool = self.pool.lock().await;
        if let Some(pool) = pool.as_ref() {
            let tx = pool.begin().await
                .map_err(|e| KernelError::transaction("sqlite_tx", &e.to_string()))?;

            let result = _f().await;

            match result {
                Ok(_) => {
                    tx.commit().await
                        .map_err(|e| KernelError::transaction("sqlite_tx", &e.to_string()))?;
                    Ok(())
                }
                Err(e) => {
                    tx.rollback().await
                        .map_err(|e| KernelError::transaction("sqlite_tx", &e.to_string()))?;
                    Err(e)
                }
            }
        } else {
            Ok(())
        }
    }
}