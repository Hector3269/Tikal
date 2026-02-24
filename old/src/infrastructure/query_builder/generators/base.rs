use super::config::GeneratorConfig;
use super::sql_generator::SqlGenerator;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

#[derive(Clone)]
pub struct BaseGenerator {
    config: GeneratorConfig,
}

impl BaseGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    pub fn mysql() -> Self {
        Self::new(GeneratorConfig::mysql())
    }

    pub fn postgres() -> Self {
        Self::new(GeneratorConfig::postgres())
    }

    pub fn sqlite() -> Self {
        Self::new(GeneratorConfig::sqlite())
    }

    pub fn config(&self) -> &GeneratorConfig {
        &self.config
    }
}

impl SqlGenerator for BaseGenerator {
    fn placeholder(&self, index: usize) -> String {
        self.config.placeholder(index)
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        self.config.quote_identifier(identifier)
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        self.config.map_type(col_type)
    }

    fn primary_key_suffix(&self) -> String {
        self.config.primary_key_suffix.to_string()
    }

    fn table_options(&self) -> String {
        self.config.table_options.to_string()
    }

    fn generate_create_table(&self, table: &TableDefinition) -> String {
        if self.config.name == "PostgreSQL" {
            return self.generate_postgres_create_table(table);
        }

        if self.config.name == "SQLite" {
            return self.generate_sqlite_create_table(table);
        }

        self.generate_standard_create_table(table)
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        let quoted = self.quote_identifier(table_name);

        if self.config.name == "PostgreSQL" {
            format!("DROP TABLE IF EXISTS {} CASCADE", quoted)
        } else {
            format!("DROP TABLE IF EXISTS {}", quoted)
        }
    }
}

impl BaseGenerator {
    fn generate_standard_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (",
            self.quote_identifier(&table.name)
        );

        let mut column_defs = Vec::new();
        for col in &table.columns {
            let mut def = format!(
                "{} {}",
                self.quote_identifier(&col.name),
                self.map_type(&col.column_type)
            );

            if col.primary_key {
                def.push_str(" PRIMARY KEY");
                if col.column_type == ColumnType::Id {
                    def.push_str(&self.primary_key_suffix());
                }
            } else {
                if !col.nullable {
                    def.push_str(" NOT NULL");
                }
                if col.unique {
                    def.push_str(" UNIQUE");
                }
            }

            if let Some(default) = &col.default_value {
                def.push_str(&format!(" DEFAULT {}", default));
            }

            column_defs.push(def);
        }

        sql.push_str(&column_defs.join(", "));
        sql.push_str(")");
        sql.push_str(&self.table_options());
        sql
    }

    fn generate_postgres_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (",
            self.quote_identifier(&table.name)
        );

        let mut column_defs = Vec::new();
        for col in &table.columns {
            let mut def = String::new();
            def.push_str(&self.quote_identifier(&col.name));
            def.push(' ');

            if col.auto_increment && matches!(col.column_type, ColumnType::Int | ColumnType::Id) {
                let sql_type = match col.column_type {
                    ColumnType::Int => "SERIAL",
                    ColumnType::Id => "BIGSERIAL",
                    _ => self.map_type(&col.column_type),
                };
                def.push_str(sql_type);
            } else {
                def.push_str(self.map_type(&col.column_type));
            }

            if col.primary_key {
                def.push_str(" PRIMARY KEY");
            } else {
                if !col.nullable {
                    def.push_str(" NOT NULL");
                }
                if col.unique {
                    def.push_str(" UNIQUE");
                }
            }

            if let Some(default) = &col.default_value {
                def.push_str(&format!(" DEFAULT {}", default));
            }

            column_defs.push(def);
        }

        sql.push_str(&column_defs.join(", "));
        sql.push_str(")");
        sql.push_str(&self.table_options());
        sql
    }

    fn generate_sqlite_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (",
            self.quote_identifier(&table.name)
        );

        let mut column_defs = Vec::new();
        for col in &table.columns {
            let mut def = format!(
                "{} {}",
                self.quote_identifier(&col.name),
                self.map_type(&col.column_type)
            );

            if col.auto_increment
                && col.primary_key
                && matches!(col.column_type, ColumnType::Int | ColumnType::Id)
            {
                def.push_str(" PRIMARY KEY");
                def.push_str(&self.primary_key_suffix());
            } else if col.primary_key {
                def.push_str(" PRIMARY KEY");
            }

            if !col.nullable && !col.primary_key {
                def.push_str(" NOT NULL");
            }

            if col.unique && !col.primary_key {
                def.push_str(" UNIQUE");
            }

            if let Some(default) = &col.default_value {
                def.push_str(&format!(" DEFAULT {}", default));
            }

            column_defs.push(def);
        }

        sql.push_str(&column_defs.join(", "));
        sql.push_str(")");
        sql.push_str(&self.table_options());
        sql
    }
}
