/// Represents whether a column can be null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nullable(bool);

impl Nullable {
    pub fn new(is_nullable: bool) -> Self {
        Self(is_nullable)
    }

    pub fn is_nullable(&self) -> bool {
        self.0
    }
}