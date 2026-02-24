use super::{super::SqlGenerator, types};
use crate::infrastructure::schema::ColumnType;

#[derive(Clone)]
pub struct SqliteGenerator;

impl SqliteGenerator {
    fn type_mapper() -> impl Fn(&ColumnType) -> &'static str {
        types::build_type_map(|col_type| match col_type {
            ColumnType::Id => Some("INTEGER"),
            ColumnType::Text => Some("TEXT"),
            ColumnType::LongText => Some("TEXT"),
            ColumnType::Int => Some("INTEGER"),
            ColumnType::Float => Some("REAL"),
            _ => None,
        })
    }
}

impl SqlGenerator for SqliteGenerator {
    fn placeholder(&self, _index: usize) -> String {
        "?".to_string()
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        Self::type_mapper()(col_type)
    }

    fn primary_key_suffix(&self) -> String {
        " AUTOINCREMENT".to_string()
    }
}
