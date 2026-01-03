use crate::domain::TikalResult;

pub trait Validate {
    fn validate(&self) -> TikalResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    Required {
        field: String,
    },
    Length {
        field: String,
        min: Option<usize>,
        max: Option<usize>,
        actual: usize,
    },
    Range {
        field: String,
        min: Option<i64>,
        max: Option<i64>,
        actual: i64,
    },
    Custom {
        field: String,
        message: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Required { field } => write!(f, "Field '{}' is required", field),
            ValidationError::Length {
                field,
                min,
                max,
                actual,
            } => {
                write!(f, "Field '{}' length {} is out of range", field, actual)?;
                if let Some(min) = min {
                    write!(f, " (min: {})", min)?;
                }
                if let Some(max) = max {
                    write!(f, " (max: {})", max)?;
                }
                Ok(())
            }
            ValidationError::Range {
                field,
                min,
                max,
                actual,
            } => {
                write!(f, "Field '{}' value {} is out of range", field, actual)?;
                if let Some(min) = min {
                    write!(f, " (min: {})", min)?;
                }
                if let Some(max) = max {
                    write!(f, " (max: {})", max)?;
                }
                Ok(())
            }
            ValidationError::Custom { field, message } => {
                write!(f, "Field '{}' validation failed: {}", field, message)
            }
        }
    }
}

impl std::error::Error for ValidationError {}
