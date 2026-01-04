use super::super::types::*;
use super::column_builder::ColumnBuilder;
use super::index_builder::IndexBuilder;

pub struct TableBuilder {
    pub(super) table: TableDefinition,
}

impl TableBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            table: TableDefinition {
                name: name.into(),
                columns: Vec::new(),
                indexes: Vec::new(),
            },
        }
    }

    pub fn id(self) -> Self {
        self.column("id", ColumnType::Id)
            .primary_key()
            .auto_increment()
            .finish()
    }

    pub fn column(self, name: impl Into<String>, column_type: ColumnType) -> ColumnBuilder {
        ColumnBuilder::new(self, name.into(), column_type)
    }

    pub fn timestamps(mut self) -> Self {
        self.table.columns.push(ColumnDefinition {
            name: "created_at".to_string(),
            column_type: ColumnType::DateTime,
            nullable: false,
            primary_key: false,
            auto_increment: false,
            default_value: None,
            unique: false,
        });

        self.table.columns.push(ColumnDefinition {
            name: "updated_at".to_string(),
            column_type: ColumnType::DateTime,
            nullable: true,
            primary_key: false,
            auto_increment: false,
            default_value: None,
            unique: false,
        });

        self
    }

    pub fn soft_deletes(mut self) -> Self {
        self.table.columns.push(ColumnDefinition {
            name: "deleted_at".to_string(),
            column_type: ColumnType::DateTime,
            nullable: true,
            primary_key: false,
            auto_increment: false,
            default_value: None,
            unique: false,
        });

        self
    }

    pub(super) fn add_column(mut self, column: ColumnDefinition) -> Self {
        self.table.columns.push(column);
        self
    }

    pub fn index(self, columns: Vec<String>) -> IndexBuilder {
        IndexBuilder::new(self, columns, false)
    }

    pub fn unique_index(self, columns: Vec<String>) -> IndexBuilder {
        IndexBuilder::new(self, columns, true)
    }

    pub(super) fn add_index(mut self, index: IndexDefinition) -> Self {
        self.table.indexes.push(index);
        self
    }

    pub fn build(self) -> TableDefinition {
        self.table
    }

    pub fn simple(name: impl Into<String>) -> Self {
        Self::new(name).id()
    }

    pub fn with_timestamps(name: impl Into<String>) -> Self {
        Self::new(name).id().timestamps()
    }

    pub fn with_all(name: impl Into<String>) -> Self {
        Self::new(name).id().timestamps().soft_deletes()
    }
}
