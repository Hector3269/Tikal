use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use sqlx::{Column, Row};
use std::collections::HashMap;

pub trait ParameterBinder<'q, DB: sqlx::Database> {
    fn bind_param(
        query: sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>>,
        param: Value,
    ) -> sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>>;

    fn bind_params(
        mut query: sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>>,
        params: Vec<Value>,
    ) -> sqlx::query::Query<'q, DB, <DB as sqlx::Database>::Arguments<'q>> {
        for param in params {
            query = Self::bind_param(query, param);
        }
        query
    }
}

pub trait RowMapper<DB: sqlx::Database> {
    fn map_value(row: &DB::Row, column_name: &str) -> TikalResult<Value>
    where
        for<'r> &'r str: sqlx::ColumnIndex<DB::Row>;

    fn map_row(row: DB::Row) -> TikalResult<HashMap<String, Value>>
    where
        for<'r> &'r str: sqlx::ColumnIndex<DB::Row>,
    {
        let mut map = HashMap::new();
        for column in row.columns() {
            let name = column.name();
            let value = Self::map_value(&row, name)?;
            map.insert(name.to_string(), value);
        }
        Ok(map)
    }

    fn map_rows(rows: Vec<DB::Row>) -> TikalResult<Vec<HashMap<String, Value>>>
    where
        for<'r> &'r str: sqlx::ColumnIndex<DB::Row>,
    {
        rows.into_iter().map(Self::map_row).collect()
    }
}
