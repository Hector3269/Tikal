use super::*;
use crate::kernel::types::schema::ColumnType;

pub struct StandardSql;

impl SqlGenerator for StandardSql {
    fn create_table(&self, table: &Table) -> String {
        let mut parts = Vec::new();

        for col in &table.columns {
            let mut def = format!("{} {}", col.name, self.column_type(&col.column_type));

            if !col.nullable.is_nullable() {
                def.push_str(" NOT NULL");
            }

            if let Some(default) = &col.default_value {
                def.push_str(&format!(" DEFAULT {}", default));
            }

            if col.is_primary_key {
                def.push_str(" PRIMARY KEY");
            }

            parts.push(def);
        }

        for fk in &table.foreign_keys {
            let local = fk.local_columns.join(", ");
            let foreign = fk.referenced_columns.join(", ");

            parts.push(format!(
                "FOREIGN KEY ({}) REFERENCES {} ({})",
                local, fk.referenced_table, foreign
            ));
        }

        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n{}\n);",
            table.name,
            parts.join(",\n")
        )
    }

    fn drop_table(&self, table: &str) -> String {
        format!("DROP TABLE IF EXISTS {};", table)
    }

    fn add_column(&self, table: &str, column: &Column) -> String {
        format!(
            "ALTER TABLE {} ADD COLUMN {} {};",
            table,
            column.name,
            self.column_type(&column.column_type)
        )
    }

    fn create_index(&self, table: &str, index: &Index) -> String {
        let unique = if index.unique { "UNIQUE " } else { "" };
        let cols = index.columns.join(", ");

        format!(
            "CREATE {}INDEX {} ON {} ({});",
            unique, index.name, table, cols
        )
    }

    fn column_type(&self, ty: &ColumnType) -> &'static str {
        match ty {
            ColumnType::Integer => "INTEGER",
            ColumnType::BigInteger => "BIGINT",
            ColumnType::String => "VARCHAR(255)",
            ColumnType::Text => "TEXT",
            ColumnType::Boolean => "BOOLEAN",
            ColumnType::Float => "REAL",
            ColumnType::Double => "DOUBLE PRECISION",
            ColumnType::Date => "DATE",
            ColumnType::DateTime => "TIMESTAMP",
            ColumnType::Time => "TIME",
            ColumnType::Binary => "BLOB",
        }
    }
}
