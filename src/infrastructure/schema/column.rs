use crate::infrastructure::core::types::*;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: ColumnName,
    pub column_type: ColumnType,
    pub nullable: Nullable,
    pub default_value: Option<DefaultValue>,
    pub is_primary_key: bool,
}

impl Column {
    pub fn new(name: ColumnName, column_type: ColumnType) -> Self {
        Self {
            name,
            column_type,
            nullable: Nullable::new(false),
            default_value: None,
            is_primary_key: false,
        }
    }

    pub fn nullable(mut self) -> Self {
        self.nullable = Nullable::new(true);
        self
    }

    pub fn default(mut self, value: DefaultValue) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self
    }
}
