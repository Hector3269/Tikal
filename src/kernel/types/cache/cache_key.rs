use super::non_empty_string::NonEmptyString;

/// Represents a key for caching query results.
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