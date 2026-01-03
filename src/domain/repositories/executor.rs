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

    async fn fetch_one(&self, sql: &str, params: Vec<Value>)
        -> TikalResult<HashMap<String, Value>>;

    async fn fetch_optional(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>>;

    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;

    async fn execute_with_rows(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;

    async fn begin_transaction(&self) -> TikalResult<Box<dyn Transaction>>;

    async fn ping(&self) -> TikalResult<bool>;

    fn driver_info(&self) -> DriverInfo;
}

#[async_trait]
pub trait Transaction: Send + Sync {
    async fn fetch_all(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>>;

    async fn fetch_one(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>>;

    async fn fetch_optional(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>>;

    async fn execute(&mut self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;

    async fn commit(self: Box<Self>) -> TikalResult<()>;

    async fn rollback(self: Box<Self>) -> TikalResult<()>;
}

#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub driver_type: DriverType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DriverType {
    PostgreSQL,
    MySQL,
    SQLite,
    MSSQL,
    Oracle,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: std::time::Duration,
    pub idle_timeout: Option<std::time::Duration>,
    pub max_lifetime: Option<std::time::Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 1,
            connect_timeout: std::time::Duration::from_secs(30),
            idle_timeout: Some(std::time::Duration::from_secs(600)),
            max_lifetime: Some(std::time::Duration::from_secs(1800)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryStats {
    pub execution_time: std::time::Duration,
    pub rows_returned: usize,
    pub rows_affected: u64,
    pub success: bool,
    pub error_message: Option<String>,
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

#[derive(Debug)]
pub struct MockExecutor {
    pub responses: Vec<Vec<HashMap<String, Value>>>,
    pub execute_responses: Vec<u64>,
    pub should_fail: bool,
}

impl MockExecutor {
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            execute_responses: Vec::new(),
            should_fail: false,
        }
    }

    pub fn with_responses(responses: Vec<Vec<HashMap<String, Value>>>) -> Self {
        Self {
            responses,
            execute_responses: Vec::new(),
            should_fail: false,
        }
    }

    pub fn with_execute_responses(execute_responses: Vec<u64>) -> Self {
        Self {
            responses: Vec::new(),
            execute_responses,
            should_fail: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            responses: Vec::new(),
            execute_responses: Vec::new(),
            should_fail: true,
        }
    }
}

#[async_trait]
impl QueryExecutor for MockExecutor {
    async fn fetch_all(
        &self,
        _sql: &str,
        _params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        if self.should_fail {
            return Err(crate::domain::error::TikalError::database_error(
                "Mock executor failure",
                "Mock executor configured to fail",
                None,
            ));
        }

        if !self.responses.is_empty() {
            Ok(self.responses[0].clone())
        } else {
            Ok(Vec::new())
        }
    }

    async fn fetch_one(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        if let Some(row) = results.into_iter().next() {
            Ok(row)
        } else {
            Err(crate::domain::error::TikalError::database_error(
                "No rows returned",
                "Expected exactly one row but got none",
                None,
            ))
        }
    }

    async fn fetch_optional(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>> {
        let results = self.fetch_all(sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&self, _sql: &str, _params: Vec<Value>) -> TikalResult<u64> {
        if self.should_fail {
            return Err(crate::domain::error::TikalError::database_error(
                "Mock executor failure",
                "Mock executor configured to fail",
                None,
            ));
        }

        if !self.execute_responses.is_empty() {
            Ok(self.execute_responses[0])
        } else {
            Ok(1)
        }
    }

    async fn execute_with_rows(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        self.execute(sql, params).await
    }

    async fn begin_transaction(&self) -> TikalResult<Box<dyn Transaction>> {
        Ok(Box::new(MockTransaction::new()))
    }

    async fn ping(&self) -> TikalResult<bool> {
        Ok(!self.should_fail)
    }

    fn driver_info(&self) -> DriverInfo {
        DriverInfo {
            name: "Mock".to_string(),
            version: "1.0.0".to_string(),
            driver_type: DriverType::SQLite,
        }
    }
}

#[derive(Debug)]
pub struct MockTransaction {
    committed: bool,
    rolled_back: bool,
}

impl MockTransaction {
    pub fn new() -> Self {
        Self {
            committed: false,
            rolled_back: false,
        }
    }
}

#[async_trait]
impl Transaction for MockTransaction {
    async fn fetch_all(
        &mut self,
        _sql: &str,
        _params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        Ok(Vec::new())
    }

    async fn fetch_one(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        if let Some(row) = results.into_iter().next() {
            Ok(row)
        } else {
            Err(crate::domain::error::TikalError::database_error(
                "No rows returned",
                "Expected exactly one row but got none",
                None,
            ))
        }
    }

    async fn fetch_optional(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>> {
        let results = self.fetch_all(sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&mut self, _sql: &str, _params: Vec<Value>) -> TikalResult<u64> {
        Ok(1)
    }

    async fn commit(mut self: Box<Self>) -> TikalResult<()> {
        self.committed = true;
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> TikalResult<()> {
        self.rolled_back = true;
        Ok(())
    }
}

pub mod utils {
    use super::*;

    pub async fn execute_batch<E: QueryExecutor>(
        executor: &E,
        queries: Vec<(String, Vec<Value>)>,
    ) -> TikalResult<Vec<u64>> {
        let mut results = Vec::new();

        for (sql, params) in queries {
            let rows_affected = executor.execute(&sql, params).await?;
            results.push(rows_affected);
        }

        Ok(results)
    }

    pub async fn execute_with_retry<E: QueryExecutor>(
        executor: &E,
        sql: &str,
        params: Vec<Value>,
        max_retries: usize,
    ) -> TikalResult<u64> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match executor.execute(sql, params.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        tokio::time::sleep(std::time::Duration::from_millis(
                            100 * (attempt + 1) as u64,
                        ))
                        .await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            crate::domain::error::TikalError::internal_error(
                "Unexpected error in retry logic",
                None,
            )
        }))
    }
}
