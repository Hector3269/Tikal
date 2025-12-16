use crate::kernel::types::core::non_empty_string::NonEmptyString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverName(NonEmptyString);

impl DriverName {
    pub fn new(name: String) -> Option<Self> {
        NonEmptyString::new(name).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}