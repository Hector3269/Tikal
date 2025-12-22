use crate::kernel::types::schema::IndexName;

#[derive(Debug, Clone)]
pub struct Index {
    pub name: IndexName,
    pub columns: Vec<String>,
    pub unique: bool,
}

impl Index {
    pub fn new(name: IndexName, columns: Vec<String>) -> Self {
        Self {
            name,
            columns,
            unique: false,
        }
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}
