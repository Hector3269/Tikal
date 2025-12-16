use super::table_name::TableName;

/// Represents a pivot table for many-to-many relationships.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PivotTable(TableName);

impl PivotTable {
    pub fn new(table: TableName) -> Self {
        Self(table)
    }

    pub fn table(&self) -> &TableName {
        &self.0
    }
}