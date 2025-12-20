use async_trait::async_trait;
use super::DatabaseDriver;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;
use crate::infrastructure::types::DbResult;

pub struct PostgresDriver;

#[async_trait]
impl DatabaseDriver for PostgresDriver {
    async fn connect(&self) -> DbResult<()> {
        Ok(())
    }

    async fn execute(&self, _sql: &str, _params: &[Value]) -> DbResult<()> {
        Ok(())
    }

    async fn query(&self, _sql: &str, _params: &[Value]) -> DbResult<Vec<DbRow>> {
        Ok(Vec::new())
    }

    async fn transaction<F, Fut>(&self, _f: F) -> DbResult<()>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = DbResult<()>> + Send,
    {
        Ok(())
    }
}