#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnName(String);

impl ColumnName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
