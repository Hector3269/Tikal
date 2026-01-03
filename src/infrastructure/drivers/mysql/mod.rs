use super::traits::QueryExecutor;
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;
use sqlx::{Column, MySqlPool, Row};
use std::collections::HashMap;

pub struct MySqlExecutor {
    pool: MySqlPool,
}

impl MySqlExecutor {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QueryExecutor for MySqlExecutor {
    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = bind_mysql_param(query, param);
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
            query = bind_mysql_param(query, param);
        }

        let rows = query.fetch_all(&self.pool).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut map = HashMap::new();
            for column in row.columns() {
                let name = column.name();
                let value = map_mysql_row_value(&row, name)?;
                map.insert(name.to_string(), value);
            }
            results.push(map);
        }

        Ok(results)
    }
}

fn bind_mysql_param<'a>(
    query: sqlx::query::Query<'a, sqlx::MySql, sqlx::mysql::MySqlArguments>,
    param: Value,
) -> sqlx::query::Query<'a, sqlx::MySql, sqlx::mysql::MySqlArguments> {
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

fn map_mysql_row_value(row: &sqlx::mysql::MySqlRow, column_name: &str) -> TikalResult<Value> {
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
