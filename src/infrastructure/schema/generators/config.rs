use super::super::types::ColumnType;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DdlConfig {
    pub name: &'static str,
    pub quote_char: char,
    pub type_mappings: HashMap<ColumnType, &'static str>,
    pub auto_increment_syntax: AutoIncrementStyle,
    pub table_options: &'static str,
    pub supports_cascade: bool,
}

#[derive(Clone, Debug)]
pub enum AutoIncrementStyle {
    Suffix(&'static str),
    TypeBased {
        int_type: &'static str,
        bigint_type: &'static str,
    },
    SuffixWithKeyword(&'static str),
}

impl DdlConfig {
    pub fn mysql() -> Self {
        let mut type_mappings = Self::default_type_mappings();
        type_mappings.insert(ColumnType::Id, "BIGINT");
        type_mappings.insert(ColumnType::Text, "VARCHAR(255)");
        type_mappings.insert(ColumnType::LongText, "LONGTEXT");
        type_mappings.insert(ColumnType::Float, "DOUBLE");
        type_mappings.insert(ColumnType::Bool, "TINYINT(1)");
        type_mappings.insert(ColumnType::Binary, "LONGBLOB");

        Self {
            name: "MySQL",
            quote_char: '`',
            type_mappings,
            auto_increment_syntax: AutoIncrementStyle::Suffix("AUTO_INCREMENT"),
            table_options: " ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
            supports_cascade: true,
        }
    }

    pub fn postgres() -> Self {
        let mut type_mappings = Self::default_type_mappings();
        type_mappings.insert(ColumnType::Id, "BIGINT");
        type_mappings.insert(ColumnType::Float, "DOUBLE PRECISION");
        type_mappings.insert(ColumnType::DateTime, "TIMESTAMP WITH TIME ZONE");
        type_mappings.insert(ColumnType::NaiveDateTime, "TIMESTAMP");
        type_mappings.insert(ColumnType::Json, "JSONB");
        type_mappings.insert(ColumnType::Binary, "BYTEA");

        Self {
            name: "PostgreSQL",
            quote_char: '"',
            type_mappings,
            auto_increment_syntax: AutoIncrementStyle::TypeBased {
                int_type: "SERIAL",
                bigint_type: "BIGSERIAL",
            },
            table_options: ";",
            supports_cascade: true,
        }
    }

    pub fn sqlite() -> Self {
        let mut type_mappings = Self::default_type_mappings();
        type_mappings.insert(ColumnType::Text, "TEXT");
        type_mappings.insert(ColumnType::LongText, "TEXT");
        type_mappings.insert(ColumnType::BigInt, "INTEGER");
        type_mappings.insert(ColumnType::Bool, "INTEGER");
        type_mappings.insert(ColumnType::DateTime, "TEXT");
        type_mappings.insert(ColumnType::NaiveDateTime, "TEXT");
        type_mappings.insert(ColumnType::Json, "TEXT");

        Self {
            name: "SQLite",
            quote_char: '"',
            type_mappings,
            auto_increment_syntax: AutoIncrementStyle::SuffixWithKeyword("AUTOINCREMENT"),
            table_options: ";",
            supports_cascade: false,
        }
    }

    fn default_type_mappings() -> HashMap<ColumnType, &'static str> {
        let mut map = HashMap::new();
        map.insert(ColumnType::Id, "INTEGER");
        map.insert(ColumnType::Text, "TEXT");
        map.insert(ColumnType::LongText, "TEXT");
        map.insert(ColumnType::Int, "INTEGER");
        map.insert(ColumnType::BigInt, "BIGINT");
        map.insert(ColumnType::Float, "REAL");
        map.insert(ColumnType::Bool, "BOOLEAN");
        map.insert(ColumnType::DateTime, "DATETIME");
        map.insert(ColumnType::NaiveDateTime, "DATETIME");
        map.insert(ColumnType::Json, "JSON");
        map.insert(ColumnType::Binary, "BLOB");
        map
    }

    pub fn quote_identifier(&self, identifier: &str) -> String {
        format!("{}{}{}", self.quote_char, identifier, self.quote_char)
    }

    pub fn map_type(&self, col_type: &ColumnType) -> &'static str {
        self.type_mappings.get(col_type).copied().unwrap_or("TEXT")
    }

    pub fn auto_increment_sql(&self, col: &super::super::types::ColumnDefinition) -> String {
        if !col.auto_increment {
            return String::new();
        }

        match &self.auto_increment_syntax {
            AutoIncrementStyle::Suffix(keyword) => {
                format!(" {}", keyword)
            }
            AutoIncrementStyle::TypeBased { .. } => String::new(),
            AutoIncrementStyle::SuffixWithKeyword(keyword) => {
                if col.primary_key {
                    format!(" {}", keyword)
                } else {
                    String::new()
                }
            }
        }
    }

    pub fn column_type_sql(&self, col: &super::super::types::ColumnDefinition) -> String {
        match &self.auto_increment_syntax {
            AutoIncrementStyle::TypeBased {
                int_type,
                bigint_type,
            } if col.auto_increment => match col.column_type {
                ColumnType::Int => int_type.to_string(),
                ColumnType::Id | ColumnType::BigInt => bigint_type.to_string(),
                _ => self.map_type(&col.column_type).to_string(),
            },
            _ => self.map_type(&col.column_type).to_string(),
        }
    }
}
