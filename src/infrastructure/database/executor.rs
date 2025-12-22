use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;

use super::drivers::DatabaseDriver;
use crate::domain::QueryExecutor;
use crate::infrastructure::types::DbResult;
use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;

pub struct SqlxQueryExecutor<D: DatabaseDriver> {
    driver: Arc<D>,
    pool: Pool<D::DB>,
}

impl<D: DatabaseDriver> SqlxQueryExecutor<D> {
    pub async fn new(driver: Arc<D>, url: &str) -> DbResult<Self> {
        let pool = driver.connect(url).await?;
        Ok(Self { driver, pool })
    }
}

#[async_trait]
impl<D: DatabaseDriver + Send + Sync> QueryExecutor for SqlxQueryExecutor<D> {
    async fn execute_raw(&self, sql: &str, params: &[Value]) -> Result<(), KernelError> {
        self.driver
            .execute(&self.pool, sql, params)
            .await
            .map_err(|e| e.into())
    }

    async fn query_raw(&self, sql: &str, params: &[Value]) -> Result<Vec<DbRow>, KernelError> {
        self.driver
            .query(&self.pool, sql, params)
            .await
            .map_err(|e| e.into())
    }

    async fn transaction(
        &self,
        f: Box<
            dyn FnOnce() -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), KernelError>> + Send>,
                > + Send,
        >,
    ) -> Result<(), KernelError> {
        self.driver
            .transaction(&self.pool, f)
            .await
            .map_err(|e| e.into())
    }

    async fn savepoint(
        &self,
        _name: &str,
        f: Box<
            dyn FnOnce() -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), KernelError>> + Send>,
                > + Send,
        >,
    ) -> Result<(), KernelError> {
        self.driver
            .transaction(&self.pool, f)
            .await
            .map_err(|e| e.into())
    }

    async fn query_stream(
        &self,
        sql: &str,
        params: &[Value],
        callback: Box<dyn Fn(DbRow) -> Result<(), KernelError> + Send + Sync>,
    ) -> Result<(), KernelError> {
        let rows = self.query_raw(sql, params).await?;
        for row in rows {
            callback(row)?;
        }
        Ok(())
    }
}
