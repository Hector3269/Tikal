use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_string())
    }
}

pub type DbRow = HashMap<String, Value>;

pub type DbResult<T> = Result<T, crate::TikalError>;
pub type OptionalDbValue<T> = Option<T>;

// Schema types
pub type TableName = String;
pub type ColumnName = String;
pub type IndexName = String;

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Integer,
    BigInteger,
    Float,
    Double,
    String(Option<usize>),
    Text,
    Boolean,
    Date,
    DateTime,
    Time,
    Binary,
    Json,
}

#[derive(Debug, Clone)]
pub struct Nullable(pub bool);

impl Nullable {
    pub fn new(nullable: bool) -> Self {
        Self(nullable)
    }

    pub fn is_nullable(&self) -> bool {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum DefaultValue {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub table: TableName,
    pub column: ColumnName,
    pub referenced_table: TableName,
    pub referenced_column: ColumnName,
}

pub mod query {
    // Query-related types
    #[derive(Debug, Clone)]
    pub struct QueryOptions {
        pub limit: Option<u64>,
        pub offset: Option<u64>,
    }
}

pub mod relation {
    // Relation-related types
    #[derive(Debug, Clone)]
    pub enum RelationType {
        HasOne,
        HasMany,
        BelongsTo,
        BelongsToMany,
    }
}

pub mod pagination {
    // Pagination-related types
    #[derive(Debug, Clone)]
    pub struct PaginationMeta {
        pub current_page: u64,
        pub per_page: u64,
        pub total: u64,
        pub total_pages: u64,
        pub has_next: bool,
        pub has_prev: bool,
    }

    #[derive(Debug, Clone)]
    pub struct Paginated<T> {
        pub data: Vec<T>,
        pub meta: PaginationMeta,
    }
}

pub mod cache {
    // Cache-related types
    #[derive(Debug, Clone)]
    pub enum CacheStrategy {
        Lru,
        Ttl,
        NoCache,
    }
}

pub mod db {
    // Database-related types
    pub use super::{DbResult, DbRow};
}

pub mod core {
    // Core types
    pub use super::Value;
}
