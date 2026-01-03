use super::*;

pub struct TableBuilder {
    table: TableDefinition,
}

impl TableBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            table: TableDefinition {
                name: name.to_string(),
                columns: Vec::new(),
                indexes: Vec::new(),
            },
        }
    }

    pub fn id(mut self) -> Self {
        self.table.columns.push(ColumnDefinition {
            name: "id".to_string(),
            column_type: ColumnType::Id,
            nullable: false,
            primary_key: true,
            auto_increment: true,
            default_value: None,
            unique: false,
        });
        self
    }

    pub fn column(self, name: &str, column_type: ColumnType) -> ColumnBuilder {
        ColumnBuilder {
            builder: self,
            column: ColumnDefinition {
                name: name.to_string(),
                column_type,
                nullable: false,
                primary_key: false,
                auto_increment: false,
                default_value: None,
                unique: false,
            },
        }
    }

    pub fn build(self) -> TableDefinition {
        self.table
    }
}

pub struct ColumnBuilder {
    builder: TableBuilder,
    column: ColumnDefinition,
}

impl ColumnBuilder {
    pub fn nullable(mut self) -> Self {
        self.column.nullable = true;
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

    pub fn finish(mut self) -> TableBuilder {
        self.builder.table.columns.push(self.column);
        self.builder
    }
}
