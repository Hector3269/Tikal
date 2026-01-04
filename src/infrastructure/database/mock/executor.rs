use crate::domain::repositories::executor::{QueryExecutor, Transaction};
use crate::domain::repositories::types::{DriverInfo, DriverType};
use crate::domain::value_objects::Value;
use crate::domain::{TikalError, TikalResult};
use async_trait::async_trait;
use std::collections::HashMap;

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
            return Err(TikalError::database_error(
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
            Err(TikalError::database_error(
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
            return Err(TikalError::database_error(
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
            Err(TikalError::database_error(
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