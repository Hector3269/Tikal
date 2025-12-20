use async_trait::async_trait;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;
use crate::infrastructure::types::DbResult;

#[async_trait]
pub trait DatabaseDriver {
    async fn connect(&self) -> DbResult<()>;
    async fn execute(&self, sql: &str, params: &[Value]) -> DbResult<()>;
    async fn query(&self, sql: &str, params: &[Value]) -> DbResult<Vec<DbRow>>;
    async fn transaction<F, Fut>(&self, f: F) -> DbResult<()>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = DbResult<()>> + Send;
}