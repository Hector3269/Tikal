/// Represents a timeout duration for queries in seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryTimeout(u32);

impl QueryTimeout {
    pub fn new(seconds: u32) -> Self {
        Self(seconds)
    }

    pub fn seconds(&self) -> u32 {
        self.0
    }
}