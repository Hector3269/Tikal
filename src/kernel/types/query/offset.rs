#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offset(u32);

impl Offset {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}