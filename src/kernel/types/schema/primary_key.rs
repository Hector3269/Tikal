#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimaryKey(i64);

impl PrimaryKey {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}