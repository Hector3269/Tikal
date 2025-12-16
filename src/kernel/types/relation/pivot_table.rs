use crate::kernel::types::schema::table_name::TableName;

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