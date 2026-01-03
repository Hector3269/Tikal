use crate::domain::TikalResult;
use crate::infrastructure::drivers::traits::QueryExecutor;
use async_trait::async_trait;

#[async_trait]
pub trait Migration: Send + Sync {
    async fn up(&self, executor: &dyn QueryExecutor) -> TikalResult<()>;
    async fn down(&self, executor: &dyn QueryExecutor) -> TikalResult<()>;
    fn name(&self) -> &str;
    fn version(&self) -> u64;
}

pub mod manager;
pub mod runner;
