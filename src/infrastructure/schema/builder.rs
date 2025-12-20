use crate::kernel::types::schema::ColumnType;
use crate::infrastructure::types::DbResult;
use super::Table;
use std::collections::HashMap;

pub struct SchemaBuilder {
    tables: HashMap<String, Table>,
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, table: Table) {
        self.tables.insert(table.name.to_string(), table);
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    pub fn tables(&self) -> &HashMap<String, Table> {
        &self.tables
    }

    pub fn generate_create_table_sql(&self, table: &Table) -> String {
        let mut sql = format!("CREATE TABLE {} (\n", table.name);

        let mut column_defs = Vec::new();

        for column in &table.columns {
            let mut col_def = format!("  {} {}", column.name, self.column_type_to_sql(&column.column_type));

            if !column.nullable.is_nullable() {
                col_def.push_str(" NOT NULL");
            }

            if let Some(default) = &column.default_value {
                col_def.push_str(&format!(" DEFAULT {}", default));
            }

            if column.is_primary_key {
                col_def.push_str(" PRIMARY KEY");
            }

            column_defs.push(col_def);
        }

        for fk in &table.foreign_keys {
            let local_cols = fk.local_columns.iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let ref_cols = fk.referenced_columns.iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", ");

            let fk_def = format!("  FOREIGN KEY ({}) REFERENCES {} ({})",
                local_cols, fk.referenced_table, ref_cols);

            column_defs.push(fk_def);
        }

        sql.push_str(&column_defs.join(",\n"));
        sql.push_str("\n);");

        sql
    }

    pub fn generate_drop_table_sql(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS {};", table_name)
    }

    pub fn generate_add_column_sql(&self, table_name: &str, column: &super::Column) -> String {
        let mut sql = format!("ALTER TABLE {} ADD COLUMN {} {}",
            table_name, column.name, self.column_type_to_sql(&column.column_type));

        if !column.nullable.is_nullable() {
            sql.push_str(" NOT NULL");
        }

        if let Some(default) = &column.default_value {
            sql.push_str(&format!(" DEFAULT {}", default));
        }

        sql.push(';');
        sql
    }

    pub fn generate_create_index_sql(&self, table_name: &str, index: &super::Index) -> String {
        let columns = index.columns.iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let unique = if index.unique { "UNIQUE " } else { "" };

        format!("CREATE {}INDEX {} ON {} ({});",
            unique, index.name, table_name, columns)
    }

    fn column_type_to_sql(&self, column_type: &ColumnType) -> String {
        match column_type {
            ColumnType::Integer => "INTEGER".to_string(),
            ColumnType::BigInteger => "BIGINT".to_string(),
            ColumnType::String => "VARCHAR(255)".to_string(),
            ColumnType::Text => "TEXT".to_string(),
            ColumnType::Boolean => "BOOLEAN".to_string(),
            ColumnType::Float => "REAL".to_string(),
            ColumnType::Double => "DOUBLE PRECISION".to_string(),
            ColumnType::Date => "DATE".to_string(),
            ColumnType::DateTime => "DATETIME".to_string(),
            ColumnType::Time => "TIME".to_string(),
            ColumnType::Binary => "BLOB".to_string(),
        }
    }
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ExecutableSchema {
    builder: SchemaBuilder,
}

impl ExecutableSchema {
    pub fn new(builder: SchemaBuilder) -> Self {
        Self { builder }
    }

    pub async fn execute<D: super::super::database::drivers::DatabaseDriver>(&self, driver: &D) -> DbResult<()> {
        for table in self.builder.tables().values() {
            let sql = self.builder.generate_create_table_sql(table);
            driver.execute(&sql, &[]).await?;
        }
        Ok(())
    }

    pub async fn execute_migration_up<M: super::super::migrations::Migration>(&self, _migration: &M) -> DbResult<()> {
        Ok(())
    }

    pub async fn execute_migration_down<M: super::super::migrations::Migration>(&self, _migration: &M) -> DbResult<()> {
        Ok(())
    }
}