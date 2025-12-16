use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnValue(Value);

impl ColumnValue {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &Value {
        &self.0
    }
}