use super::super::types::*;
use super::table_builder::TableBuilder;

pub struct ColumnBuilder {
    table_builder: TableBuilder,
    column: ColumnDefinition,
}

impl ColumnBuilder {
    pub(super) fn new(table_builder: TableBuilder, name: String, column_type: ColumnType) -> Self {
        Self {
            table_builder,
            column: ColumnDefinition {
                name,
                column_type,
                nullable: false,
                primary_key: false,
                auto_increment: false,
                default_value: None,
                unique: false,
            },
        }
    }

    pub fn nullable(mut self) -> Self {
        self.column.nullable = true;
        self
    }

    pub fn not_null(mut self) -> Self {
        self.column.nullable = false;
        self
    }

    pub fn primary_key(mut self) -> Self {
        self.column.primary_key = true;
        self.column.nullable = false;
        self
    }

    pub fn auto_increment(mut self) -> Self {
        self.column.auto_increment = true;
        self
    }

    pub fn unique(mut self) -> Self {
        self.column.unique = true;
        self
    }

    pub fn default_value(mut self, value: impl Into<crate::domain::value_objects::Value>) -> Self {
        self.column.default_value = Some(value.into());
        self
    }

    pub fn default(self, value: impl Into<crate::domain::value_objects::Value>) -> Self {
        self.default_value(value)
    }

    pub fn finish(self) -> TableBuilder {
        self.table_builder.add_column(self.column)
    }
}
