use super::column_name::ColumnName;
use super::sort_direction::SortDirection;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderBy {
    column: ColumnName,
    direction: SortDirection,
}

impl OrderBy {
    pub fn new(column: ColumnName, direction: SortDirection) -> Self {
        Self { column, direction }
    }

    pub fn column(&self) -> &ColumnName {
        &self.column
    }

    pub fn direction(&self) -> SortDirection {
        self.direction
    }
}