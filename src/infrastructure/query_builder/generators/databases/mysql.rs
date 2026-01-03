use super::{super::SqlGenerator, types};
use crate::infrastructure::schema::ColumnType;

#[derive(Clone)]
pub struct MySqlGenerator;

impl MySqlGenerator {
    fn type_mapper() -> impl Fn(&ColumnType) -> &'static str {
        types::build_type_map(|col_type| match col_type {
            ColumnType::Id => Some("BIGINT"),
            ColumnType::Text => Some("VARCHAR(255)"),
            ColumnType::LongText => Some("LONGTEXT"),
            ColumnType::Int => Some("INT"),
            ColumnType::Float => Some("DOUBLE"),
            ColumnType::Binary => Some("LONGBLOB"),
            _ => None,
        })
    }
}

impl SqlGenerator for MySqlGenerator {
    fn placeholder(&self, _index: usize) -> String {
        "?".to_string()
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        format!("`{}`", identifier)
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        Self::type_mapper()(col_type)
    }

    fn primary_key_suffix(&self) -> String {
        " AUTO_INCREMENT".to_string()
    }

    fn table_options(&self) -> String {
        " ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci".to_string()
    }
}
