use std::collections::HashMap;
use crate::kernel::types::core::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct DbRow {
    pub columns: HashMap<String, Value>,
}

impl DbRow {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Value) {
        self.columns.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.columns.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.columns.remove(key)
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }
}

impl Default for DbRow {
    fn default() -> Self {
        Self::new()
    }
}