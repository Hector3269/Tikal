use crate::domain::repositories::types::{DriverInfo, DriverType};
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use sqlx::Sqlite;

pub struct SqliteBinder;
pub struct SqliteMapper;

pub type SqliteExecutor = super::DatabaseExecutor<Sqlite, SqliteBinder, SqliteMapper>;
pub type SqliteTransaction = super::DatabaseTransaction<Sqlite, SqliteBinder, SqliteMapper>;

impl SqliteExecutor {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        let driver_info = DriverInfo {
            name: "SQLite".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            driver_type: DriverType::SQLite,
        };
        Self {
            pool,
            driver_info,
            _binder: std::marker::PhantomData,
            _mapper: std::marker::PhantomData,
        }
    }
}

impl<'q> super::ParameterBinder<'q, Sqlite> for SqliteBinder {
    fn bind_param(
        query: sqlx::query::Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
        param: Value,
    ) -> sqlx::query::Query<'q, Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
        match param {
            Value::Null => query.bind(None::<String>),
            Value::Text(s) => query.bind(s),
            Value::Int(i) => query.bind(i),
            Value::Float(f) => query.bind(f.into_inner()),
            Value::Bool(b) => query.bind(b),
            Value::DateTime(dt) => query.bind(dt.to_rfc3339()),
            Value::Json(j) => query.bind(j.to_string()),
            Value::Binary(b) => query.bind(b),
            Value::NaiveDateTime(ndt) => query.bind(ndt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

impl super::RowMapper<Sqlite> for SqliteMapper {
    fn map_value(row: &sqlx::sqlite::SqliteRow, column_name: &str) -> TikalResult<Value> {
        use sqlx::Row;

        if let Ok(value) = row.try_get::<i64, _>(column_name) {
            return Ok(Value::Int(value));
        }
        if let Ok(value) = row.try_get::<f64, _>(column_name) {
            return Ok(Value::Float(ordered_float::OrderedFloat(value)));
        }
        if let Ok(value) = row.try_get::<bool, _>(column_name) {
            return Ok(Value::Bool(value));
        }
        if let Ok(value) = row.try_get::<Vec<u8>, _>(column_name) {
            return Ok(Value::Binary(value));
        }
        if let Ok(value) = row.try_get::<String, _>(column_name) {
            if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&value) {
                return Ok(Value::DateTime(dt.with_timezone(&chrono::Utc)));
            }
            if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S") {
                return Ok(Value::NaiveDateTime(ndt));
            }
            if value.starts_with('{') || value.starts_with('[') {
                if let Ok(json) = serde_json::from_str(&value) {
                    return Ok(Value::Json(json));
                }
            }
            return Ok(Value::Text(value));
        }

        Ok(Value::Null)
    }
}
