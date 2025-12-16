#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: String) -> Option<Self> {
        if s.is_empty() { None } else { Some(Self(s)) }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}