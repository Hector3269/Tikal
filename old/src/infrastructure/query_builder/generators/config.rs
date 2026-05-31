use crate::infrastructure::schema::ColumnType;

#[derive(Clone)]
pub struct GeneratorConfig {
    pub name: &'static str,
    pub placeholder_style: PlaceholderStyle,
    pub quote_char: QuoteStyle,
    pub type_mapper: TypeMapper,
    pub primary_key_suffix: &'static str,
    pub table_options: &'static str,
}

#[derive(Clone)]
pub enum PlaceholderStyle {
    Dollar,
    Question,
}

#[derive(Clone)]
pub enum QuoteStyle {
    DoubleQuote,
    Backtick,
    DoubleQuoteSqlite,
}

#[derive(Clone)]
pub struct TypeMapper {
    custom_mappings: std::collections::HashMap<ColumnType, &'static str>,
}

impl TypeMapper {
    pub fn new() -> Self {
        Self {
            custom_mappings: std::collections::HashMap::new(),
        }
    }

    pub fn with_mapping(mut self, col_type: ColumnType, sql_type: &'static str) -> Self {
        self.custom_mappings.insert(col_type, sql_type);
        self
    }

    pub fn map(&self, col_type: &ColumnType) -> &'static str {
        if let Some(&custom) = self.custom_mappings.get(col_type) {
            return custom;
        }

        match col_type {
            ColumnType::Id => "INTEGER",
            ColumnType::Text => "VARCHAR(255)",
            ColumnType::LongText => "TEXT",
            ColumnType::Int => "INTEGER",
            ColumnType::BigInt => "BIGINT",
            ColumnType::Float => "REAL",
            ColumnType::Bool => "BOOLEAN",
            ColumnType::DateTime => "DATETIME",
            ColumnType::NaiveDateTime => "DATETIME",
            ColumnType::Json => "JSON",
            ColumnType::Binary => "BLOB",
        }
    }
}

impl Default for TypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorConfig {
    pub fn mysql() -> Self {
        Self {
            name: "MySQL",
            placeholder_style: PlaceholderStyle::Question,
            quote_char: QuoteStyle::Backtick,
            type_mapper: TypeMapper::new()
                .with_mapping(ColumnType::Id, "BIGINT")
                .with_mapping(ColumnType::Text, "VARCHAR(255)")
                .with_mapping(ColumnType::LongText, "LONGTEXT")
                .with_mapping(ColumnType::Float, "DOUBLE")
                .with_mapping(ColumnType::Binary, "LONGBLOB"),
            primary_key_suffix: " AUTO_INCREMENT",
            table_options: " ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci",
        }
    }

    pub fn postgres() -> Self {
        Self {
            name: "PostgreSQL",
            placeholder_style: PlaceholderStyle::Dollar,
            quote_char: QuoteStyle::DoubleQuote,
            type_mapper: TypeMapper::new()
                .with_mapping(ColumnType::Id, "SERIAL")
                .with_mapping(ColumnType::Float, "DOUBLE PRECISION")
                .with_mapping(ColumnType::DateTime, "TIMESTAMP WITH TIME ZONE")
                .with_mapping(ColumnType::NaiveDateTime, "TIMESTAMP")
                .with_mapping(ColumnType::Json, "JSONB")
                .with_mapping(ColumnType::Binary, "BYTEA"),
            primary_key_suffix: "",
            table_options: ";",
        }
    }

    pub fn sqlite() -> Self {
        Self {
            name: "SQLite",
            placeholder_style: PlaceholderStyle::Question,
            quote_char: QuoteStyle::DoubleQuoteSqlite,
            type_mapper: TypeMapper::new()
                .with_mapping(ColumnType::Id, "INTEGER")
                .with_mapping(ColumnType::Text, "TEXT")
                .with_mapping(ColumnType::LongText, "TEXT")
                .with_mapping(ColumnType::BigInt, "INTEGER")
                .with_mapping(ColumnType::Bool, "INTEGER")
                .with_mapping(ColumnType::DateTime, "TEXT")
                .with_mapping(ColumnType::NaiveDateTime, "TEXT")
                .with_mapping(ColumnType::Json, "TEXT")
                .with_mapping(ColumnType::Binary, "BLOB"),
            primary_key_suffix: " AUTOINCREMENT",
            table_options: ";",
        }
    }

    pub fn placeholder(&self, index: usize) -> String {
        match self.placeholder_style {
            PlaceholderStyle::Dollar => format!("${}", index + 1),
            PlaceholderStyle::Question => "?".to_string(),
        }
    }

    pub fn quote_identifier(&self, identifier: &str) -> String {
        if identifier == "*" {
            return "*".to_string();
        }

        match self.quote_char {
            QuoteStyle::DoubleQuote | QuoteStyle::DoubleQuoteSqlite => {
                format!("\"{}\"", identifier)
            }
            QuoteStyle::Backtick => format!("`{}`", identifier),
        }
    }

    pub fn map_type(&self, col_type: &ColumnType) -> &'static str {
        self.type_mapper.map(col_type)
    }
}
