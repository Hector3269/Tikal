use super::DdlGenerator;
use crate::infrastructure::schema::{ColumnDefinition, ColumnType, TableDefinition};

pub struct SqliteDdlGenerator;

impl SqliteDdlGenerator {
    fn column_type_to_sql(&self, column_type: &ColumnType) -> &'static str {
        match column_type {
            ColumnType::Text => "TEXT",
            ColumnType::LongText => "TEXT",
            ColumnType::Int => "INTEGER",
            ColumnType::BigInt => "INTEGER",
            ColumnType::Float => "REAL",
            ColumnType::Bool => "INTEGER", // SQLite uses INTEGER for boolean
            ColumnType::DateTime => "TEXT", // SQLite stores datetime as TEXT
            ColumnType::NaiveDateTime => "TEXT",
            ColumnType::Json => "TEXT", // SQLite stores JSON as TEXT
            ColumnType::Binary => "BLOB",
            ColumnType::Id => "INTEGER",
        }
    }

    fn build_column_definition(&self, column: &ColumnDefinition) -> String {
        let mut parts = vec![format!("\"{}\"", column.name)];

        parts.push(self.column_type_to_sql(&column.column_type).to_string());

        if column.primary_key {
            parts.push("PRIMARY KEY".to_string());
        }

        if column.auto_increment {
            if column.primary_key && matches!(column.column_type, ColumnType::Int) {
                parts.push("AUTOINCREMENT".to_string());
            }
        }

        if !column.nullable && !column.primary_key {
            parts.push("NOT NULL".to_string());
        }

        if column.unique && !column.primary_key {
            parts.push("UNIQUE".to_string());
        }

        if let Some(default) = &column.default_value {
            parts.push(format!("DEFAULT {}", default));
        }

        parts.join(" ")
    }
}

impl DdlGenerator for SqliteDdlGenerator {
    fn generate_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!("CREATE TABLE \"{}\" (\n", table.name);

        let column_defs: Vec<String> = table
            .columns
            .iter()
            .map(|col| format!("  {}", self.build_column_definition(col)))
            .collect();

        sql.push_str(&column_defs.join(",\n"));
        sql.push_str("\n);");

        sql
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS \"{}\";", table_name)
    }

    fn generate_create_index(
        &self,
        table_name: &str,
        index_name: &str,
        columns: &[String],
        unique: bool,
    ) -> String {
        let unique_str = if unique { "UNIQUE " } else { "" };
        let column_list = columns
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "CREATE {}INDEX \"{}\" ON \"{}\" ({});",
            unique_str, index_name, table_name, column_list
        )
    }

    fn generate_drop_index(&self, index_name: &str) -> String {
        format!("DROP INDEX IF EXISTS \"{}\";", index_name)
    }
}
