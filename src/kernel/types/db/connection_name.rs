use crate::kernel::types::core::non_empty_string::NonEmptyString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionName(NonEmptyString);

impl ConnectionName {
    pub fn new(s: String) -> Option<Self> {
        NonEmptyString::new(s).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}