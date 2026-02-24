use crate::domain::repositories::executor::{QueryExecutor, Transaction};
use crate::domain::repositories::types::{DriverInfo, DriverType};
use crate::domain::value_objects::Value;
use crate::domain::{TikalError, TikalResult};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MockExecutor {
    responses: Vec<Vec<HashMap<String, Value>>>,
    execute_responses: Vec<u64>,
    current_query_index: std::sync::Arc<std::sync::Mutex<usize>>,
    current_execute_index: std::sync::Arc<std::sync::Mutex<usize>>,
    should_fail: bool,
    fail_message: Option<String>,
}

impl MockExecutor {
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            execute_responses: Vec::new(),
            current_query_index: std::sync::Arc::new(std::sync::Mutex::new(0)),
            current_execute_index: std::sync::Arc::new(std::sync::Mutex::new(0)),
            should_fail: false,
            fail_message: None,
        }
    }

    pub fn with_query_response(mut self, response: Vec<HashMap<String, Value>>) -> Self {
        self.responses.push(response);
        self
    }

    pub fn with_query_responses(mut self, responses: Vec<Vec<HashMap<String, Value>>>) -> Self {
        self.responses = responses;
        self
    }

    pub fn with_execute_response(mut self, rows: u64) -> Self {
        self.execute_responses.push(rows);
        self
    }

    pub fn with_execute_responses(mut self, responses: Vec<u64>) -> Self {
        self.execute_responses = responses;
        self
    }

    pub fn failing(mut self) -> Self {
        self.should_fail = true;
        self
    }

    pub fn failing_with(mut self, message: impl Into<String>) -> Self {
        self.should_fail = true;
        self.fail_message = Some(message.into());
        self
    }

    pub fn reset(&self) {
        *self.current_query_index.lock().unwrap() = 0;
        *self.current_execute_index.lock().unwrap() = 0;
    }

    fn get_failure_error(&self) -> TikalError {
        let message = self
            .fail_message
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Mock executor configured to fail");
        TikalError::database_error("Mock executor failure", message, None)
    }

    fn get_next_query_response(&self) -> TikalResult<Vec<HashMap<String, Value>>> {
        if self.should_fail {
            return Err(self.get_failure_error());
        }

        let mut index = self.current_query_index.lock().unwrap();
        if *index < self.responses.len() {
            let response = self.responses[*index].clone();
            *index += 1;
            Ok(response)
        } else {
            Ok(Vec::new())
        }
    }

    fn get_next_execute_response(&self) -> TikalResult<u64> {
        if self.should_fail {
            return Err(self.get_failure_error());
        }

        let mut index = self.current_execute_index.lock().unwrap();
        if *index < self.execute_responses.len() {
            let rows = self.execute_responses[*index];
            *index += 1;
            Ok(rows)
        } else {
            Ok(1) // Default: 1 row affected
        }
    }
}

impl Default for MockExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QueryExecutor for MockExecutor {
    async fn fetch_all(
        &self,
        _sql: &str,
        _params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        self.get_next_query_response()
    }

    async fn fetch_one(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        results.into_iter().next().ok_or_else(|| {
            TikalError::database_error(
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

    async fn execute(&self, _sql: &str, _params: Vec<Value>) -> TikalResult<u64> {
        self.get_next_execute_response()
    }

    async fn execute_with_rows(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        self.execute(sql, params).await
    }

    async fn begin_transaction(&self) -> TikalResult<Box<dyn Transaction>> {
        if self.should_fail {
            return Err(self.get_failure_error());
        }
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
    responses: Vec<Vec<HashMap<String, Value>>>,
    execute_responses: Vec<u64>,
    current_query_index: usize,
    current_execute_index: usize,
    committed: bool,
    rolled_back: bool,
    should_fail: bool,
}

impl MockTransaction {
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            execute_responses: Vec::new(),
            current_query_index: 0,
            current_execute_index: 0,
            committed: false,
            rolled_back: false,
            should_fail: false,
        }
    }

    pub fn with_query_response(mut self, response: Vec<HashMap<String, Value>>) -> Self {
        self.responses.push(response);
        self
    }

    pub fn with_execute_response(mut self, rows: u64) -> Self {
        self.execute_responses.push(rows);
        self
    }

    pub fn failing(mut self) -> Self {
        self.should_fail = true;
        self
    }

    pub fn is_committed(&self) -> bool {
        self.committed
    }

    pub fn is_rolled_back(&self) -> bool {
        self.rolled_back
    }
}

impl Default for MockTransaction {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transaction for MockTransaction {
    async fn fetch_all(
        &mut self,
        _sql: &str,
        _params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        if self.should_fail {
            return Err(TikalError::database_error(
                "Mock transaction failure",
                "Mock transaction configured to fail",
                None,
            ));
        }

        if self.current_query_index < self.responses.len() {
            let response = self.responses[self.current_query_index].clone();
            self.current_query_index += 1;
            Ok(response)
        } else {
            Ok(Vec::new())
        }
    }

    async fn fetch_one(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        results.into_iter().next().ok_or_else(|| {
            TikalError::database_error(
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

    async fn execute(&mut self, _sql: &str, _params: Vec<Value>) -> TikalResult<u64> {
        if self.should_fail {
            return Err(TikalError::database_error(
                "Mock transaction failure",
                "Mock transaction configured to fail",
                None,
            ));
        }

        if self.current_execute_index < self.execute_responses.len() {
            let rows = self.execute_responses[self.current_execute_index];
            self.current_execute_index += 1;
            Ok(rows)
        } else {
            Ok(1)
        }
    }

    async fn commit(mut self: Box<Self>) -> TikalResult<()> {
        if self.should_fail {
            return Err(TikalError::database_error(
                "Mock transaction failure",
                "Mock transaction configured to fail on commit",
                None,
            ));
        }
        self.committed = true;
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> TikalResult<()> {
        if self.should_fail {
            return Err(TikalError::database_error(
                "Mock transaction failure",
                "Mock transaction configured to fail on rollback",
                None,
            ));
        }
        self.rolled_back = true;
        Ok(())
    }
}
