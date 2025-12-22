use super::Table;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SchemaBuilder {
    tables: HashMap<String, Table>,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_table(&mut self, table: Table) -> &mut Self {
        self.tables.insert(table.name.to_string(), table);
        self
    }

    pub fn table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    pub fn tables(&self) -> impl Iterator<Item = &Table> {
        self.tables.values()
    }
}
