use super::super::types::{ColumnDefinition, TableDefinition};
use super::config::DdlConfig;
use super::r#trait::DdlGenerator;

#[derive(Clone)]
pub struct UnifiedDdlGenerator {
    config: DdlConfig,
}

impl UnifiedDdlGenerator {
    pub fn new(config: DdlConfig) -> Self {
        Self { config }
    }

    pub fn mysql() -> Self {
        Self::new(DdlConfig::mysql())
    }

    pub fn postgres() -> Self {
        Self::new(DdlConfig::postgres())
    }

    pub fn sqlite() -> Self {
        Self::new(DdlConfig::sqlite())
    }

    pub fn from_driver(driver: &str) -> Option<Self> {
        match driver {
            "mysql" => Some(Self::mysql()),
            "postgres" | "postgresql" => Some(Self::postgres()),
            "sqlite" => Some(Self::sqlite()),
            _ => None,
        }
    }

    pub fn config(&self) -> &DdlConfig {
        &self.config
    }
}

impl DdlGenerator for UnifiedDdlGenerator {
    fn generate_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = String::from("CREATE TABLE IF NOT EXISTS ");
        sql.push_str(&self.quote_identifier(&table.name));
        sql.push_str(" (\n");

        let column_defs: Vec<String> = table
            .columns
            .iter()
            .map(|col| format!("  {}", self.generate_column_definition(col)))
            .collect();

        sql.push_str(&column_defs.join(",\n"));
        sql.push_str("\n)");
        sql.push_str(self.config.table_options);

        sql
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        let mut sql = format!("DROP TABLE IF EXISTS {}", self.quote_identifier(table_name));

        if self.config.supports_cascade {
            sql.push_str(" CASCADE");
        }

        sql
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
            .map(|c| self.quote_identifier(c))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "CREATE {}INDEX {} ON {} ({})",
            unique_str,
            self.quote_identifier(index_name),
            self.quote_identifier(table_name),
            column_list
        )
    }

    fn generate_drop_index(&self, index_name: &str) -> String {
        format!("DROP INDEX IF EXISTS {}", self.quote_identifier(index_name))
    }

    fn generate_column_definition(&self, col: &ColumnDefinition) -> String {
        let mut parts = vec![self.quote_identifier(&col.name)];

        parts.push(self.config.column_type_sql(col));

        if col.primary_key {
            parts.push("PRIMARY KEY".to_string());
        }

        let auto_inc = self.config.auto_increment_sql(col);
        if !auto_inc.is_empty() {
            parts.push(auto_inc);
        }

        if !col.nullable && !col.primary_key {
            parts.push("NOT NULL".to_string());
        }

        if col.unique && !col.primary_key {
            parts.push("UNIQUE".to_string());
        }

        if let Some(default) = &col.default_value {
            parts.push(format!("DEFAULT {}", default));
        }

        parts.join(" ")
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        self.config.quote_identifier(identifier)
    }
}

pub type MySqlDdlGenerator = UnifiedDdlGenerator;
pub type PostgresDdlGenerator = UnifiedDdlGenerator;
pub type SqliteDdlGenerator = UnifiedDdlGenerator;

impl Default for MySqlDdlGenerator {
    fn default() -> Self {
        Self::mysql()
    }
}
