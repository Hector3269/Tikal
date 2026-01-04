use crate::infrastructure::schema::{ColumnDefinition, ColumnType, TableDefinition};

pub struct BaseDdlGenerator;

impl BaseDdlGenerator {
    pub fn build_column_definition_template(
        column: &ColumnDefinition,
        quote_fn: &dyn Fn(&str) -> String,
        type_map_fn: &dyn Fn(&ColumnType) -> &'static str,
        primary_key_suffix_fn: &dyn Fn() -> String,
    ) -> String {
        let mut parts = vec![quote_fn(&column.name)];

        parts.push(type_map_fn(&column.column_type).to_string());

        if column.auto_increment {
            parts.push(primary_key_suffix_fn());
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

    pub fn generate_create_table_template(
        table: &TableDefinition,
        quote_fn: &dyn Fn(&str) -> String,
        build_column_fn: &dyn Fn(&ColumnDefinition) -> String,
        table_options_fn: &dyn Fn() -> String,
    ) -> String {
        let mut sql = format!("CREATE TABLE {} (\n", quote_fn(&table.name));

        let column_defs: Vec<String> = table
            .columns
            .iter()
            .map(|col| format!("  {}", build_column_fn(col)))
            .collect();

        sql.push_str(&column_defs.join(",\n"));
        sql.push_str("\n)");
        sql.push_str(&table_options_fn());
        sql
    }

    pub fn generate_drop_table_template(
        table_name: &str,
        quote_fn: &dyn Fn(&str) -> String,
    ) -> String {
        format!("DROP TABLE IF EXISTS {};", quote_fn(table_name))
    }

    pub fn generate_create_index_template(
        table_name: &str,
        index_name: &str,
        columns: &[String],
        unique: bool,
        quote_fn: &dyn Fn(&str) -> String,
    ) -> String {
        let unique_str = if unique { "UNIQUE " } else { "" };
        let column_list = columns
            .iter()
            .map(|c| quote_fn(c))
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "CREATE {}INDEX {} ON {} ({});",
            unique_str,
            quote_fn(index_name),
            quote_fn(table_name),
            column_list
        )
    }

    pub fn generate_drop_index_template(
        index_name: &str,
        quote_fn: &dyn Fn(&str) -> String,
    ) -> String {
        format!("DROP INDEX {};", quote_fn(index_name))
    }
}
