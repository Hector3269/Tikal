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
}

use std::collections::HashMap;
