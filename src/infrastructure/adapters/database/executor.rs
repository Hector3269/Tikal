use async_trait::async_trait;
use sqlx::Pool;
use std::sync::Arc;

use super::drivers::DatabaseDriver;
use crate::domain::QueryExecutor;
use crate::infrastructure::types::{DbResult, DbRow, Value};
use crate::TikalError;

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
    async fn execute_raw(&self, sql: &str, params: &[Value]) -> Result<(), TikalError> {
        self.driver.execute(&self.pool, sql, params).await
    }

    async fn query_raw(&self, sql: &str, params: &[Value]) -> Result<Vec<DbRow>, TikalError> {
        self.driver.query(&self.pool, sql, params).await
    }

    async fn transaction(
        &self,
        f: Box<
            dyn FnOnce() -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), TikalError>> + Send>,
                > + Send,
        >,
    ) -> Result<(), TikalError> {
        self.driver.transaction(&self.pool, f).await
    }

    async fn savepoint(
        &self,
        _name: &str,
        f: Box<
            dyn FnOnce() -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), TikalError>> + Send>,
                > + Send,
        >,
    ) -> Result<(), TikalError> {
        self.driver.transaction(&self.pool, f).await
    }

    async fn query_stream(
        &self,
        sql: &str,
        params: &[Value],
        callback: Box<dyn Fn(DbRow) -> Result<(), TikalError> + Send + Sync>,
    ) -> Result<(), TikalError> {
        let rows = self.query_raw(sql, params).await?;
        for row in rows {
            callback(row)?;
        }
        Ok(())
    }
}
