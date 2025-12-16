/// Represents the time-to-live for cached items in seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ttl(u32);

impl Ttl {
    pub fn new(seconds: u32) -> Self {
        Self(seconds)
    }

    pub fn seconds(&self) -> u32 {
        self.0
    }
}