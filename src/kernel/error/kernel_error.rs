use std::fmt;

#[derive(Debug)]
pub enum KernelError {
    InvalidState {
        message: String,
    },
    Configuration {
        message: String,
    },
    NotImplemented {
        feature: &'static str,
    },
    InternalError {
        message: String,
    },
    InvariantViolation {
        message: String,
    },
    DatabaseError {
        source: String,
    },
    ValidationError {
        field: String,
        message: String,
    },
    ConnectionError {
        driver: String,
        message: String,
    },
    QueryError {
        sql: String,
        message: String,
    },
    MappingError {
        entity: String,
        message: String,
    },
    TransactionError {
        transaction_id: String,
        message: String,
    },
    RelationError {
        relation: String,
        message: String,
    },
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::InvalidState { message } => write!(f, "Invalid state: {}", message),
            KernelError::Configuration { message } => write!(f, "Configuration error: {}", message),
            KernelError::NotImplemented { feature } => {
                write!(f, "Feature not implemented: {}", feature)
            }
            KernelError::InternalError { message } => write!(f, "Internal error: {}", message),
            KernelError::InvariantViolation { message } => {
                write!(f, "Invariant violation: {}", message)
            }
            KernelError::DatabaseError { source } => write!(f, "Database error: {}", source),
            KernelError::ValidationError { field, message } => {
                write!(f, "Validation error on '{}': {}", field, message)
            }
            KernelError::ConnectionError { driver, message } => {
                write!(f, "Connection error ({}): {}", driver, message)
            }
            KernelError::QueryError { sql, message } => {
                write!(f, "Query error ({}): {}", sql, message)
            }
            KernelError::MappingError { entity, message } => {
                write!(f, "Mapping error ({}): {}", entity, message)
            }
            KernelError::TransactionError {
                transaction_id,
                message,
            } => write!(f, "Transaction error ({}): {}", transaction_id, message),
            KernelError::RelationError { relation, message } => {
                write!(f, "Relation error ({}): {}", relation, message)
            }
        }
    }
}

impl std::error::Error for KernelError {}

impl From<sqlx::Error> for KernelError {
    fn from(err: sqlx::Error) -> Self {
        KernelError::DatabaseError {
            source: err.to_string(),
        }
    }
}

impl KernelError {
    pub fn invalid_state(msg: &str) -> Self {
        KernelError::InvalidState {
            message: msg.to_string(),
        }
    }

    pub fn config(msg: &str) -> Self {
        KernelError::Configuration {
            message: msg.to_string(),
        }
    }

    pub fn validation(field: &str, msg: &str) -> Self {
        KernelError::ValidationError {
            field: field.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn internal(msg: &str) -> Self {
        KernelError::InternalError {
            message: msg.to_string(),
        }
    }

    pub fn db(source: &str) -> Self {
        KernelError::DatabaseError {
            source: source.to_string(),
        }
    }

    pub fn connection(driver: &str, msg: &str) -> Self {
        KernelError::ConnectionError {
            driver: driver.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn query(sql: &str, msg: &str) -> Self {
        KernelError::QueryError {
            sql: sql.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn mapping(entity: &str, msg: &str) -> Self {
        KernelError::MappingError {
            entity: entity.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn transaction(tx_id: &str, msg: &str) -> Self {
        KernelError::TransactionError {
            transaction_id: tx_id.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn relation(relation: &str, msg: &str) -> Self {
        KernelError::RelationError {
            relation: relation.to_string(),
            message: msg.to_string(),
        }
    }

    pub fn not_implemented(feature: &'static str) -> Self {
        KernelError::NotImplemented { feature }
    }
}
