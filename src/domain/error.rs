use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TikalError {
    #[error("Invalid state: {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    InvalidState {
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },
    #[error("Configuration error: {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    Configuration {
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Feature not implemented: {feature}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    NotImplemented {
        feature: &'static str,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Internal error: {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    InternalError {
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Database error: {db_message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    DatabaseError {
        db_message: String,
        context: Option<String>,
        error_code: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Validation error on '{field}': {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    ValidationError {
        field: String,
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Connection error ({driver}): {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    ConnectionError {
        driver: String,
        message: String,
        context: Option<String>,
        retry_count: Option<u32>,
        backtrace: Option<Backtrace>,
    },

    #[error("Query error: {message} (SQL: {sql}){}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    QueryError {
        sql: String,
        message: String,
        context: Option<String>,
        params_count: Option<usize>,
        backtrace: Option<Backtrace>,
    },

    #[error("Mapping error ({entity}): {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    MappingError {
        entity: String,
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Transaction error ({transaction_id}): {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    TransactionError {
        transaction_id: String,
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Record not found: {entity} with ID {id}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    RecordNotFound {
        entity: String,
        id: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Unique constraint violation ({constraint}): {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    UniqueConstraintViolation {
        constraint: String,
        message: String,
        context: Option<String>,
        conflicting_value: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Foreign key violation ({constraint}): {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    ForeignKeyViolation {
        constraint: String,
        message: String,
        context: Option<String>,
        referenced_table: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Infrastructure error: {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    Infrastructure {
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("SQL injection attempt: {reason} (input: {input}){}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    SqlInjectionAttempt {
        input: String,
        reason: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Connection timeout ({driver}) after {duration_ms}ms{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    ConnectionTimeout {
        driver: String,
        duration_ms: u64,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Query timeout after {duration_ms}ms: {sql}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    QueryTimeout {
        sql: String,
        duration_ms: u64,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Migration lock failed for '{migration}'{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    MigrationLockFailed {
        migration: String,
        holder: Option<String>,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("NULL constraint violation: column '{column}' in table '{table}' cannot be NULL{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    NullConstraintViolation {
        column: String,
        table: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },

    #[error("Error: {message}{}", .context.as_ref().map(|c| format!(" (context: {c})")).unwrap_or_default())]
    GenericError {
        message: String,
        context: Option<String>,
        backtrace: Option<Backtrace>,
    },
}

impl From<sqlx::Error> for TikalError {
    fn from(err: sqlx::Error) -> Self {
        let error_code = match &err {
            sqlx::Error::Database(db_err) => db_err.code().map(|c| c.to_string()),
            _ => None,
        };

        TikalError::DatabaseError {
            db_message: err.to_string(),
            context: None,
            error_code,
            backtrace: Some(Backtrace::capture()),
        }
    }
}
impl TikalError {
    pub fn invalid_state(msg: &str) -> Self {
        TikalError::InvalidState {
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn config(msg: &str) -> Self {
        TikalError::Configuration {
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn validation(field: &str, msg: &str) -> Self {
        TikalError::ValidationError {
            field: field.to_string(),
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn internal(msg: &str) -> Self {
        TikalError::InternalError {
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn db(message: &str) -> Self {
        TikalError::DatabaseError {
            db_message: message.to_string(),
            context: None,
            error_code: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn database_error(message: &str, context: &str, error_code: Option<String>) -> Self {
        TikalError::DatabaseError {
            db_message: message.to_string(),
            context: Some(context.to_string()),
            error_code,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn internal_error(message: &str, context: Option<String>) -> Self {
        TikalError::InternalError {
            message: message.to_string(),
            context,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn connection(driver: &str, msg: &str) -> Self {
        TikalError::ConnectionError {
            driver: driver.to_string(),
            message: msg.to_string(),
            context: None,
            retry_count: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn query(sql: &str, msg: &str) -> Self {
        TikalError::QueryError {
            sql: sql.to_string(),
            message: msg.to_string(),
            context: None,
            params_count: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn mapping(entity: &str, msg: &str) -> Self {
        TikalError::MappingError {
            entity: entity.to_string(),
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn transaction(tx_id: &str, msg: &str) -> Self {
        TikalError::TransactionError {
            transaction_id: tx_id.to_string(),
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn not_implemented(feature: &'static str) -> Self {
        TikalError::NotImplemented {
            feature,
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn record_not_found(entity: &str, id: &str) -> Self {
        TikalError::RecordNotFound {
            entity: entity.to_string(),
            id: id.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn unique_violation(constraint: &str, msg: &str) -> Self {
        TikalError::UniqueConstraintViolation {
            constraint: constraint.to_string(),
            message: msg.to_string(),
            context: None,
            conflicting_value: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn foreign_key_violation(constraint: &str, msg: &str) -> Self {
        TikalError::ForeignKeyViolation {
            constraint: constraint.to_string(),
            message: msg.to_string(),
            context: None,
            referenced_table: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn null_constraint_violation(column: &str, table: &str) -> Self {
        TikalError::NullConstraintViolation {
            column: column.to_string(),
            table: table.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn infrastructure(msg: &str) -> Self {
        TikalError::Infrastructure {
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn sql_injection(input: &str, reason: &str) -> Self {
        TikalError::SqlInjectionAttempt {
            input: input.to_string(),
            reason: reason.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn generic(msg: &str) -> Self {
        TikalError::GenericError {
            message: msg.to_string(),
            context: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        let ctx = context.into();
        match &mut self {
            TikalError::InvalidState { context, .. }
            | TikalError::Configuration { context, .. }
            | TikalError::NotImplemented { context, .. }
            | TikalError::InternalError { context, .. }
            | TikalError::DatabaseError { context, .. }
            | TikalError::ValidationError { context, .. }
            | TikalError::ConnectionError { context, .. }
            | TikalError::QueryError { context, .. }
            | TikalError::MappingError { context, .. }
            | TikalError::TransactionError { context, .. }
            | TikalError::RecordNotFound { context, .. }
            | TikalError::UniqueConstraintViolation { context, .. }
            | TikalError::ForeignKeyViolation { context, .. }
            | TikalError::Infrastructure { context, .. }
            | TikalError::SqlInjectionAttempt { context, .. }
            | TikalError::ConnectionTimeout { context, .. }
            | TikalError::QueryTimeout { context, .. }
            | TikalError::MigrationLockFailed { context, .. }
            | TikalError::NullConstraintViolation { context, .. }
            | TikalError::GenericError { context, .. } => {
                *context = Some(ctx);
            }
        }
        self
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            TikalError::ConnectionTimeout { .. }
                | TikalError::QueryTimeout { .. }
                | TikalError::ConnectionError { .. }
        )
    }

    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            TikalError::ValidationError { .. }
                | TikalError::RecordNotFound { .. }
                | TikalError::UniqueConstraintViolation { .. }
                | TikalError::ForeignKeyViolation { .. }
                | TikalError::NullConstraintViolation { .. }
        )
    }

    pub fn backtrace(&self) -> Option<&Backtrace> {
        match self {
            TikalError::InvalidState { backtrace, .. }
            | TikalError::Configuration { backtrace, .. }
            | TikalError::NotImplemented { backtrace, .. }
            | TikalError::InternalError { backtrace, .. }
            | TikalError::DatabaseError { backtrace, .. }
            | TikalError::ValidationError { backtrace, .. }
            | TikalError::ConnectionError { backtrace, .. }
            | TikalError::QueryError { backtrace, .. }
            | TikalError::MappingError { backtrace, .. }
            | TikalError::TransactionError { backtrace, .. }
            | TikalError::RecordNotFound { backtrace, .. }
            | TikalError::UniqueConstraintViolation { backtrace, .. }
            | TikalError::ForeignKeyViolation { backtrace, .. }
            | TikalError::Infrastructure { backtrace, .. }
            | TikalError::SqlInjectionAttempt { backtrace, .. }
            | TikalError::ConnectionTimeout { backtrace, .. }
            | TikalError::QueryTimeout { backtrace, .. }
            | TikalError::MigrationLockFailed { backtrace, .. }
            | TikalError::NullConstraintViolation { backtrace, .. }
            | TikalError::GenericError { backtrace, .. } => backtrace.as_ref(),
        }
    }
}

pub type TikalResult<T> = Result<T, TikalError>;
