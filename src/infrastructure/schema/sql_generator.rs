use super::{Column, Index, Table};
use crate::infrastructure::core::types::ColumnType;

pub trait SqlGenerator {
    fn create_table(&self, table: &Table) -> String;
    fn drop_table(&self, table: &str) -> String;
    fn add_column(&self, table: &str, column: &Column) -> String;
    fn create_index(&self, table: &str, index: &Index) -> String;
    fn column_type(&self, ty: &ColumnType) -> &'static str;
}
