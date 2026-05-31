use crate::domain::repositories::types::{DriverInfo, DriverType};
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use sqlx::Postgres;

pub struct PostgresBinder;
pub struct PostgresMapper;

pub type PostgresExecutor = super::DatabaseExecutor<Postgres, PostgresBinder, PostgresMapper>;
pub type PostgresTransaction = super::DatabaseTransaction<Postgres, PostgresBinder, PostgresMapper>;

impl PostgresExecutor {
    pub fn new(pool: sqlx::PgPool) -> Self {
        let driver_info = DriverInfo {
            name: "PostgreSQL".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: DriverType::PostgreSQL,
        };
        Self {
            pool,
            driver_info,
            _binder: std::marker::PhantomData,
            _mapper: std::marker::PhantomData,
        }
    }
}

impl<'q> super::ParameterBinder<'q, Postgres> for PostgresBinder {
    fn bind_param(
        query: sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments>,
        param: Value,
    ) -> sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments> {
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
}

impl super::RowMapper<Postgres> for PostgresMapper {
    fn map_value(row: &sqlx::postgres::PgRow, column_name: &str) -> TikalResult<Value> {
        use sqlx::Row;

        if let Ok(value) = row.try_get::<chrono::DateTime<chrono::Utc>, _>(column_name) {
            return Ok(Value::DateTime(value));
        }
        if let Ok(value) = row.try_get::<chrono::NaiveDateTime, _>(column_name) {
            return Ok(Value::NaiveDateTime(value));
        }
        if let Ok(value) = row.try_get::<i64, _>(column_name) {
            return Ok(Value::Int(value));
        }
        if let Ok(value) = row.try_get::<i32, _>(column_name) {
            return Ok(Value::Int(value as i64));
        }
        if let Ok(value) = row.try_get::<f64, _>(column_name) {
            return Ok(Value::Float(ordered_float::OrderedFloat(value)));
        }
        if let Ok(value) = row.try_get::<bool, _>(column_name) {
            return Ok(Value::Bool(value));
        }
        if let Ok(value) = row.try_get::<serde_json::Value, _>(column_name) {
            return Ok(Value::Json(value));
        }
        if let Ok(value) = row.try_get::<Vec<u8>, _>(column_name) {
            return Ok(Value::Binary(value));
        }
        if let Ok(value) = row.try_get::<String, _>(column_name) {
            return Ok(Value::Text(value));
        }

        Ok(Value::Null)
    }
}
