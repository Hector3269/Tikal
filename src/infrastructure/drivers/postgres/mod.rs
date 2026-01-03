use super::traits::{QueryExecutor, Transaction};
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;
use sqlx::{Column, PgPool, Row};
use std::collections::HashMap;

pub struct PostgresExecutor {
    pool: PgPool,
}

impl PostgresExecutor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QueryExecutor for PostgresExecutor {
    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = bind_pg_param(query, param);
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
            query = bind_pg_param(query, param);
        }

        let rows = query.fetch_all(&self.pool).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut map = HashMap::new();
            for column in row.columns() {
                let name = column.name();
                let value = map_pg_row_value(&row, name)?;
                map.insert(name.to_string(), value);
            }
            results.push(map);
        }

        Ok(results)
    }

    async fn begin(&self) -> TikalResult<Box<dyn Transaction>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(PostgresTransaction::new(tx)))
    }
}

pub struct PostgresTransaction {
    tx: sqlx::Transaction<'static, sqlx::Postgres>,
}

impl PostgresTransaction {
    pub fn new(tx: sqlx::Transaction<'static, sqlx::Postgres>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl Transaction for PostgresTransaction {
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
            query = bind_pg_param(query, param);
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
            query = bind_pg_param(query, param);
        }

        let rows = query.fetch_all(&mut *self.tx).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut map = HashMap::new();
            for column in row.columns() {
                let name = column.name();
                let value = map_pg_row_value(&row, name)?;
                map.insert(name.to_string(), value);
            }
            results.push(map);
        }

        Ok(results)
    }
}

pub fn bind_pg_param<'a>(
    query: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    param: Value,
) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
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

pub fn map_pg_row_value(row: &sqlx::postgres::PgRow, column_name: &str) -> TikalResult<Value> {
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
    if let Ok(value) = row.try_get::<chrono::DateTime<chrono::Utc>, _>(column_name) {
        return Ok(Value::DateTime(value));
    }

    Ok(Value::Null)
}
