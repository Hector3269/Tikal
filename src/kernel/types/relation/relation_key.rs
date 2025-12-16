use super::column_name::ColumnName;

/// Represents the key mapping for a relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationKey {
    local_column: ColumnName,
    foreign_column: ColumnName,
}

impl RelationKey {
    pub fn new(local_column: ColumnName, foreign_column: ColumnName) -> Self {
        Self {
            local_column,
            foreign_column,
        }
    }

    pub fn local_column(&self) -> &ColumnName {
        &self.local_column
    }

    pub fn foreign_column(&self) -> &ColumnName {
        &self.foreign_column
    }
}