use crate::infrastructure::schema::ColumnType;

pub fn default_text_type() -> &'static str {
    "VARCHAR(255)"
}

pub fn default_long_text_type() -> &'static str {
    "TEXT"
}

pub fn default_int_type() -> &'static str {
    "INTEGER"
}

pub fn default_bigint_type() -> &'static str {
    "BIGINT"
}

pub fn default_float_type() -> &'static str {
    "REAL"
}

pub fn default_bool_type() -> &'static str {
    "BOOLEAN"
}

pub fn default_datetime_type() -> &'static str {
    "DATETIME"
}

pub fn default_json_type() -> &'static str {
    "JSON"
}

pub fn default_binary_type() -> &'static str {
    "BLOB"
}

pub fn build_type_map<F>(customize: F) -> impl Fn(&ColumnType) -> &'static str
where
    F: Fn(&ColumnType) -> Option<&'static str>,
{
    move |col_type: &ColumnType| {
        if let Some(custom) = customize(col_type) {
            return custom;
        }
        match col_type {
            ColumnType::Id => "INTEGER",
            ColumnType::Text => default_text_type(),
            ColumnType::LongText => default_long_text_type(),
            ColumnType::Int => default_int_type(),
            ColumnType::BigInt => default_bigint_type(),
            ColumnType::Float => default_float_type(),
            ColumnType::Bool => default_bool_type(),
            ColumnType::DateTime => default_datetime_type(),
            ColumnType::NaiveDateTime => default_datetime_type(),
            ColumnType::Json => default_json_type(),
            ColumnType::Binary => default_binary_type(),
        }
    }
}
