use super::{Column, Index};
use crate::infrastructure::core::types::*;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: TableName,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub foreign_keys: Vec<ForeignKey>,
}

impl Table {
    pub fn new(name: TableName) -> Self {
        Self {
            name,
            columns: vec![],
            indexes: vec![],
            foreign_keys: vec![],
        }
    }

    pub fn column(&mut self, column: Column) -> &mut Self {
        self.columns.push(column);
        self
    }

    pub fn index(&mut self, index: Index) -> &mut Self {
        self.indexes.push(index);
        self
    }

    pub fn foreign_key(&mut self, fk: ForeignKey) -> &mut Self {
        self.foreign_keys.push(fk);
        self
    }
}
