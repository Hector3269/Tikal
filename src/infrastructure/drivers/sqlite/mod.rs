use super::traits::{QueryExecutor, Transaction};
use crate::domain::repositories::executor::{QueryExecutor as DomainQueryExecutor, Transaction as DomainTransaction};
use crate::domain::repositories::types::{DriverInfo, DriverType};
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;
use sqlx::{Column, Row, SqlitePool};
use std::collections::HashMap;

pub struct SqliteExecutor {
    pool: SqlitePool,
}

impl SqliteExecutor {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QueryExecutor for SqliteExecutor {
    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = bind_sqlite_param(query, param);
        }

        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn fetch_all(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = bind_sqlite_param(query, param);
        }

        let rows = query.fetch_all(&self.pool).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut map = HashMap::new();
            for column in row.columns() {
                let name = column.name();
                let value = map_sqlite_row_value(&row, name)?;
                map.insert(name.to_string(), value);
            }
            results.push(map);
        }

        Ok(results)
    }

    async fn begin(&self) -> TikalResult<Box<dyn Transaction>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(SqliteTransaction::new(tx)))
    }
}

#[async_trait]
impl DomainQueryExecutor for SqliteExecutor {
    async fn fetch_all(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        <Self as QueryExecutor>::fetch_all(self, sql, params).await
    }

    async fn fetch_one(&self, sql: &str, params: Vec<Value>) -> TikalResult<HashMap<String, Value>> {
        let results = <Self as super::traits::QueryExecutor>::fetch_all(self, sql, params).await?;
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
        let results = <Self as super::traits::QueryExecutor>::fetch_all(self, sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        <Self as super::traits::QueryExecutor>::execute(self, sql, params).await
    }

    async fn execute_with_rows(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        <Self as super::traits::QueryExecutor>::execute(self, sql, params).await
    }

    async fn begin_transaction(&self) -> TikalResult<Box<dyn DomainTransaction>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(SqliteTransaction::new(tx)))
    }

    async fn ping(&self) -> TikalResult<bool> {
        match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn driver_info(&self) -> DriverInfo {
        DriverInfo {
            name: "SQLite".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: DriverType::SQLite,
        }
    }
}

pub struct SqliteTransaction {
    tx: sqlx::Transaction<'static, sqlx::Sqlite>,
}

impl SqliteTransaction {
    pub fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl Transaction for SqliteTransaction {
    async fn begin(&self) -> TikalResult<Box<dyn Transaction>> {
        panic!("Cannot begin transaction on an existing transaction")
    }

    async fn commit(self: Box<Self>) -> TikalResult<()> {
        self.tx.commit().await?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> TikalResult<()> {
        self.tx.rollback().await?;
        Ok(())
    }

    async fn execute(&mut self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = bind_sqlite_param(query, param);
        }

        let result = query.execute(&mut *self.tx).await?;
        Ok(result.rows_affected())
    }

    async fn fetch_all(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = bind_sqlite_param(query, param);
        }

        let rows = query.fetch_all(&mut *self.tx).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut map = HashMap::new();
            for column in row.columns() {
                let name = column.name();
                let value = map_sqlite_row_value(&row, name)?;
                map.insert(name.to_string(), value);
            }
            results.push(map);
        }

        Ok(results)
    }
}

#[async_trait]
impl DomainTransaction for SqliteTransaction {
    async fn fetch_all(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        <Self as Transaction>::fetch_all(self, sql, params).await
    }

    async fn fetch_one(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = <Self as Transaction>::fetch_all(self, sql, params).await?;
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
        let results = <Self as Transaction>::fetch_all(self, sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&mut self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        <Self as Transaction>::execute(self, sql, params).await
    }

    async fn commit(self: Box<Self>) -> TikalResult<()> {
        <Self as Transaction>::commit(self).await
    }

    async fn rollback(self: Box<Self>) -> TikalResult<()> {
        <Self as Transaction>::rollback(self).await
    }
}

fn bind_sqlite_param<'a>(
    query: sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>>,
    param: Value,
) -> sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
    match param {
        Value::Null => query.bind(None::<String>),
        Value::Text(s) => query.bind(s),
        Value::Int(i) => query.bind(i),
        Value::Float(f) => query.bind(f.into_inner()),
        Value::Bool(b) => query.bind(b),
        Value::DateTime(dt) => query.bind(dt),
        Value::Json(j) => query.bind(j),
        Value::Binary(b) => query.bind(b),
        Value::NaiveDateTime(ndt) => query.bind(ndt),
    }
}

fn map_sqlite_row_value(row: &sqlx::sqlite::SqliteRow, column_name: &str) -> TikalResult<Value> {
    if let Ok(value) = row.try_get::<String, _>(column_name) {
        return Ok(Value::Text(value));
    }
    if let Ok(value) = row.try_get::<i64, _>(column_name) {
        return Ok(Value::Int(value));
    }
    if let Ok(value) = row.try_get::<bool, _>(column_name) {
        return Ok(Value::Bool(value));
    }
    if let Ok(value) = row.try_get::<f64, _>(column_name) {
        return Ok(Value::Float(ordered_float::OrderedFloat(value)));
    }

    Ok(Value::Null)
}
