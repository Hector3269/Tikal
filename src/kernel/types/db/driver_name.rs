use super::non_empty_string::NonEmptyString;

/// Represents the name of a database driver.
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