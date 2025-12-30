use crate::infrastructure::types::DbResult;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;
use async_trait::async_trait;
use sqlx::{Database, Pool};

#[async_trait]
pub trait DatabaseDriver: Send + Sync {
    type DB: Database;

    async fn connect(&self, url: &str) -> DbResult<Pool<Self::DB>>;
    async fn execute(&self, pool: &Pool<Self::DB>, sql: &str, params: &[Value]) -> DbResult<()>;
    async fn query(
        &self,
        pool: &Pool<Self::DB>,
        sql: &str,
        params: &[Value],
    ) -> DbResult<Vec<DbRow>>;
    async fn transaction(
        &self,
        pool: &Pool<Self::DB>,
        f: Box<
            dyn FnOnce()
                    -> std::pin::Pin<Box<dyn std::future::Future<Output = DbResult<()>> + Send>>
                + Send,
        >,
    ) -> DbResult<()>;
}
