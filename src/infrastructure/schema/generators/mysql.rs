use super::DdlGenerator;
use crate::infrastructure::schema::{ColumnDefinition, ColumnType, TableDefinition};

pub struct MySqlDdlGenerator;

impl MySqlDdlGenerator {
    fn column_type_to_sql(&self, column_type: &ColumnType) -> &'static str {
        match column_type {
            ColumnType::Text => "TEXT",
            ColumnType::LongText => "LONGTEXT",
            ColumnType::Int => "INT",
            ColumnType::BigInt => "BIGINT",
            ColumnType::Float => "DOUBLE",
            ColumnType::Bool => "TINYINT(1)",
            ColumnType::DateTime => "DATETIME",
            ColumnType::NaiveDateTime => "DATETIME",
            ColumnType::Json => "JSON",
            ColumnType::Binary => "BLOB",
            ColumnType::Id => "BIGINT",
        }
    }

    fn build_column_definition(&self, column: &ColumnDefinition) -> String {
        let mut parts = vec![format!("`{}`", column.name)];

        parts.push(self.column_type_to_sql(&column.column_type).to_string());

        if column.auto_increment {
            parts.push("AUTO_INCREMENT".to_string());
        }

        if column.primary_key {
            parts.push("PRIMARY KEY".to_string());
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

impl DdlGenerator for MySqlDdlGenerator {
    fn generate_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!("CREATE TABLE `{}` (\n", table.name);

        let column_defs: Vec<String> = table
            .columns
            .iter()
            .map(|col| format!("  {}", self.build_column_definition(col)))
            .collect();

        sql.push_str(&column_defs.join(",\n"));
        sql.push_str("\n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;");

        sql
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS `{}`;", table_name)
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
            .map(|c| format!("`{}`", c))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "CREATE {}INDEX `{}` ON `{}` ({});",
            unique_str, index_name, table_name, column_list
        )
    }

    fn generate_drop_index(&self, index_name: &str) -> String {
        format!("DROP INDEX `{}`;", index_name)
    }
}
