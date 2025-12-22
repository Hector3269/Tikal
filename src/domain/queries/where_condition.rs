use super::operators::Operator;
use crate::kernel::types::core::Value;

#[derive(Debug, Clone)]
pub struct WhereCondition {
    pub column: String,
    pub operator: Operator,
    pub value: Option<Value>,
    pub values: Option<Vec<Value>>,
}

impl WhereCondition {
    pub fn simple(column: impl Into<String>, operator: Operator, value: Value) -> Self {
        Self {
            column: column.into(),
            operator,
            value: Some(value),
            values: None,
        }
    }

    pub fn multi(column: impl Into<String>, operator: Operator, values: Vec<Value>) -> Self {
        assert!(
            operator.supports_multiple(),
            "Operator does not support multiple values"
        );

        Self {
            column: column.into(),
            operator,
            value: None,
            values: Some(values),
        }
    }

    pub fn null(column: impl Into<String>, operator: Operator) -> Self {
        Self {
            column: column.into(),
            operator,
            value: None,
            values: None,
        }
    }
}
