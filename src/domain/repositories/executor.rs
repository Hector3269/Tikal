use crate::domain::repositories::types::{DriverInfo, QueryStats};
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait QueryExecutor: Send + Sync {
    async fn fetch_all(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>>;

    async fn fetch_one(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        results.into_iter().next().ok_or_else(|| {
            crate::domain::error::TikalError::database_error(
                "No rows returned",
                "Expected exactly one row but got none",
                None,
            )
        })
    }

    async fn fetch_optional(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>> {
        let results = self.fetch_all(sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;

    async fn execute_with_rows(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;

    async fn begin_transaction(&self) -> TikalResult<Box<dyn Transaction>>;

    async fn ping(&self) -> TikalResult<bool>;

    fn driver_info(&self) -> DriverInfo;
}

#[async_trait]
pub trait Transaction: Send {
    async fn fetch_all(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>>;

    async fn fetch_one(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        results.into_iter().next().ok_or_else(|| {
            crate::domain::error::TikalError::database_error(
                "No rows returned",
                "Expected exactly one row but got none",
                None,
            )
        })
    }

    async fn fetch_optional(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>> {
        let results = self.fetch_all(sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&mut self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;

    async fn commit(self: Box<Self>) -> TikalResult<()>;

    async fn rollback(self: Box<Self>) -> TikalResult<()>;
}

#[async_trait]
pub trait QueryExecutorWithStats: QueryExecutor {
    async fn fetch_all_with_stats(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<(Vec<HashMap<String, Value>>, QueryStats)>;

    async fn execute_with_stats(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<(u64, QueryStats)>;
}
