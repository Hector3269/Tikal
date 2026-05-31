use super::super::types::*;
use super::table_builder::TableBuilder;

pub struct IndexBuilder {
    table_builder: TableBuilder,
    columns: Vec<String>,
    unique: bool,
    name: Option<String>,
}

impl IndexBuilder {
    pub(super) fn new(table_builder: TableBuilder, columns: Vec<String>, unique: bool) -> Self {
        Self {
            table_builder,
            columns,
            unique,
            name: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn finish(self) -> TableBuilder {
        let table_name = &self.table_builder.table.name;
        let index_name = self.name.unwrap_or_else(|| {
            let cols = self.columns.join("_");
            let prefix = if self.unique { "unique" } else { "idx" };
            format!("{}_{}_on_{}", prefix, table_name, cols)
        });

        let index = IndexDefinition {
            name: index_name,
            columns: self.columns,
            unique: self.unique,
        };

        self.table_builder.add_index(index)
    }
}
