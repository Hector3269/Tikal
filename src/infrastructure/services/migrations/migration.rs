use async_trait::async_trait;
use crate::infrastructure::types::DbResult;

#[async_trait]
pub trait SchemaOperations {
    async fn create_table(&self, name: &str) -> DbResult<()>;
    async fn drop_table(&self, name: &str) -> DbResult<()>;
}

#[async_trait]
pub trait Migration {
    fn name(&self) -> &'static str;

    async fn up(&self, schema: &dyn SchemaOperations) -> DbResult<()>;

    async fn down(&self, schema: &dyn SchemaOperations) -> DbResult<()>;
}