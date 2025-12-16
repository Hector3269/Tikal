use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<T> {
    value: i64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}
