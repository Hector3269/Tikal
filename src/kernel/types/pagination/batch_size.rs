#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatchSize(std::num::NonZeroU32);

impl BatchSize {
    pub fn new(value: u32) -> Option<Self> {
        std::num::NonZeroU32::new(value).map(Self)
    }

    pub fn value(&self) -> u32 {
        self.0.get()
    }
}