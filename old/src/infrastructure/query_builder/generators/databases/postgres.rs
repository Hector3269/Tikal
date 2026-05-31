use super::{super::SqlGenerator, types};
use crate::infrastructure::schema::ColumnType;

#[derive(Clone)]
pub struct PostgresGenerator;

impl PostgresGenerator {
    fn type_mapper() -> impl Fn(&ColumnType) -> &'static str {
        types::build_type_map(|col_type| match col_type {
            ColumnType::Id => Some("SERIAL"),
            ColumnType::Float => Some("DOUBLE PRECISION"),
            ColumnType::DateTime => Some("TIMESTAMP WITH TIME ZONE"),
            ColumnType::NaiveDateTime => Some("TIMESTAMP"),
            ColumnType::Json => Some("JSONB"),
            ColumnType::Binary => Some("BYTEA"),
            _ => None,
        })
    }
}

impl SqlGenerator for PostgresGenerator {
    fn placeholder(&self, index: usize) -> String {
        format!("${}", index + 1)
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        if identifier == "*" {
            "*".to_string()
        } else {
            format!("\"{}\"", identifier)
        }
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        Self::type_mapper()(col_type)
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!(
            "DROP TABLE IF EXISTS {} CASCADE",
            self.quote_identifier(table_name)
        )
    }
}
