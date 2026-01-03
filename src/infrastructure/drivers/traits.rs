use crate::domain::TikalResult;
use async_trait::async_trait;

#[async_trait]
pub trait QueryExecutor: Send + Sync {
    async fn execute(
        &self,
        sql: &str,
        params: Vec<crate::domain::value_objects::Value>,
    ) -> TikalResult<u64>;
    async fn fetch_all(
        &self,
        sql: &str,
        params: Vec<crate::domain::value_objects::Value>,
    ) -> TikalResult<Vec<HashMap<String, crate::domain::value_objects::Value>>>;
    async fn begin(&self) -> TikalResult<Box<dyn Transaction>>;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    async fn begin(&self) -> TikalResult<Box<dyn Transaction>>;
    async fn commit(self: Box<Self>) -> TikalResult<()>;
    async fn rollback(self: Box<Self>) -> TikalResult<()>;
    async fn execute(
        &mut self,
        sql: &str,
        params: Vec<crate::domain::value_objects::Value>,
    ) -> TikalResult<u64>;
    async fn fetch_all(
        &mut self,
        sql: &str,
        params: Vec<crate::domain::value_objects::Value>,
    ) -> TikalResult<Vec<HashMap<String, crate::domain::value_objects::Value>>>;
}

use std::collections::HashMap;
