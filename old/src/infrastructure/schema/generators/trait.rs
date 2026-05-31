use super::super::types::{ColumnDefinition, TableDefinition};

pub trait DdlGenerator {
    fn generate_create_table(&self, table: &TableDefinition) -> String;

    fn generate_drop_table(&self, table_name: &str) -> String;

    fn generate_create_index(
        &self,
        table_name: &str,
        index_name: &str,
        columns: &[String],
        unique: bool,
    ) -> String;

    fn generate_drop_index(&self, index_name: &str) -> String;

    fn generate_add_column(&self, table_name: &str, column: &ColumnDefinition) -> String {
        format!(
            "ALTER TABLE {} ADD COLUMN {}",
            self.quote_identifier(table_name),
            self.generate_column_definition(column)
        )
    }

    fn generate_drop_column(&self, table_name: &str, column_name: &str) -> String {
        format!(
            "ALTER TABLE {} DROP COLUMN {}",
            self.quote_identifier(table_name),
            self.quote_identifier(column_name)
        )
    }

    fn generate_column_definition(&self, column: &ColumnDefinition) -> String;

    fn quote_identifier(&self, identifier: &str) -> String;
}
