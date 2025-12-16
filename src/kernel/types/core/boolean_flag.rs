#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BooleanFlag(bool);

impl BooleanFlag {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn value(&self) -> bool {
        self.0
    }
}