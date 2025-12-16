use crate::kernel::types::core::non_empty_string::NonEmptyString;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey(NonEmptyString);

impl CacheKey {
    pub fn new(key: String) -> Option<Self> {
        NonEmptyString::new(key).map(Self)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}