use crate::kernel::types::core::value::Value;

/// Represents a default value for a column.
#[derive(Debug, Clone, PartialEq)]
pub struct DefaultValue(Value);

impl DefaultValue {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &Value {
        &self.0
    }
}

impl std::fmt::Display for DefaultValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}