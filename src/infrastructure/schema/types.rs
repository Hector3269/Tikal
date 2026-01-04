use crate::domain::value_objects::Value;

#[derive(Debug, Clone)]
pub struct SchemaDefinition {
    pub tables: Vec<TableDefinition>,
}

#[derive(Debug, Clone)]
pub struct TableDefinition {
    pub name: String,
    pub columns: Vec<ColumnDefinition>,
    pub indexes: Vec<IndexDefinition>,
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub primary_key: bool,
    pub auto_increment: bool,
    pub default_value: Option<Value>,
    pub unique: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnType {
    Id,
    Text,
    LongText,
    Int,
    BigInt,
    Float,
    Bool,
    DateTime,
    NaiveDateTime,
    Json,
    Binary,
}

#[derive(Debug, Clone)]
pub struct IndexDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}
