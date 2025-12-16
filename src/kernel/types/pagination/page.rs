/// Represents a page number for pagination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page(u32);

impl Page {
    pub fn new(number: u32) -> Self {
        Self(number)
    }

    pub fn number(&self) -> u32 {
        self.0
    }
}