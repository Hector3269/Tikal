use super::DatabaseDriver;
use crate::infrastructure::types::DbResult;
use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;
use async_trait::async_trait;
use sqlx::{Column, MySql, Pool, Row};

pub struct MySQLDriver;

#[async_trait]
impl DatabaseDriver for MySQLDriver {
    type DB = MySql;

    async fn connect(&self, url: &str) -> DbResult<Pool<Self::DB>> {
        Pool::connect(url).await.map_err(|e| e.into())
    }

    async fn execute(&self, pool: &Pool<Self::DB>, sql: &str, params: &[Value]) -> DbResult<()> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = match param {
                Value::Int(i) => query.bind(*i),
                Value::Float(f) => query.bind(*f),
                Value::Bool(b) => query.bind(*b),
                Value::String(s) => query.bind(s),
                Value::Null => query.bind(None::<String>),
            };
        }
        query.execute(pool).await.map_err(KernelError::from)?;
        Ok(())
    }

    async fn query(
        &self,
        pool: &Pool<Self::DB>,
        sql: &str,
        params: &[Value],
    ) -> DbResult<Vec<DbRow>> {
        let mut query = sqlx::query(sql);
        for param in params {
            query = match param {
                Value::Int(i) => query.bind(*i),
                Value::Float(f) => query.bind(*f),
                Value::Bool(b) => query.bind(*b),
                Value::String(s) => query.bind(s),
                Value::Null => query.bind(None::<String>),
            };
        }
        let rows = query.fetch_all(pool).await.map_err(KernelError::from)?;
        let mut result = Vec::new();
        for row in rows {
            let mut db_row = DbRow::new();
            for column in row.columns() {
                let name = column.name().to_string();
                let value = if let Ok(v) = row.try_get::<Option<String>, _>(name.as_str()) {
                    v.map(Value::String).unwrap_or(Value::Null)
                } else if let Ok(v) = row.try_get::<Option<i64>, _>(name.as_str()) {
                    v.map(Value::Int).unwrap_or(Value::Null)
                } else if let Ok(v) = row.try_get::<Option<f64>, _>(name.as_str()) {
                    v.map(Value::Float).unwrap_or(Value::Null)
                } else if let Ok(v) = row.try_get::<Option<bool>, _>(name.as_str()) {
                    v.map(Value::Bool).unwrap_or(Value::Null)
                } else {
                    Value::Null
                };
                db_row.insert(name, value);
            }
            result.push(db_row);
        }
        Ok(result)
    }

    async fn transaction(
        &self,
        pool: &Pool<Self::DB>,
        f: Box<
            dyn FnOnce()
                    -> std::pin::Pin<Box<dyn std::future::Future<Output = DbResult<()>> + Send>>
                + Send,
        >,
    ) -> DbResult<()> {
        let tx = pool.begin().await.map_err(KernelError::from)?;
        let result = f().await;
        if result.is_ok() {
            tx.commit().await.map_err(KernelError::from)?;
        } else {
            tx.rollback().await.map_err(KernelError::from)?;
        }
        result
    }
}
