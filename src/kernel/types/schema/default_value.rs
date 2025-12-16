use super::value::Value;

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