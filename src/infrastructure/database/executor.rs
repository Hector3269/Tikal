use std::sync::Arc;
use async_trait::async_trait;

use super::drivers::DatabaseDriver;
use crate::kernel::types::db::DbRow;
use crate::kernel::types::core::Value;
use crate::infrastructure::types::DbResult;


#[async_trait]
pub trait QueryExecutor: Send + Sync {
    async fn execute_raw(&self, sql: &str, params: &[Value]) -> DbResult<()>;
    async fn query_raw(&self, sql: &str, params: &[Value]) -> DbResult<Vec<DbRow>>;
    async fn transaction<F, Fut>(&self, f: F) -> DbResult<()>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = DbResult<()>> + Send;
}

pub struct SqlxQueryExecutor<D: DatabaseDriver> {
    driver: Arc<D>,
}

impl<D: DatabaseDriver> SqlxQueryExecutor<D> {
    pub fn new(driver: Arc<D>) -> Self {
        Self { driver }
    }
}

#[async_trait]
impl<D: DatabaseDriver + Send + Sync> QueryExecutor for SqlxQueryExecutor<D> {
    async fn execute_raw(&self, sql: &str, params: &[Value]) -> DbResult<()> {
        self.driver.execute(sql, params).await
    }

    async fn query_raw(&self, sql: &str, params: &[Value]) -> DbResult<Vec<DbRow>> {
        self.driver.query(sql, params).await
    }

    async fn transaction<F, Fut>(&self, f: F) -> DbResult<()>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = DbResult<()>> + Send,
    {
        self.driver.transaction(f).await
    }
}