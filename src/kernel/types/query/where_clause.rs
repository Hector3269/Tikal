use super::column_name::ColumnName;
use super::operator::Operator;
use super::value::Value;

/// Represents a single WHERE condition in a query.
#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    column: ColumnName,
    operator: Operator,
    value: Value,
}

impl WhereClause {
    pub fn new(column: ColumnName, operator: Operator, value: Value) -> Self {
        Self {
            column,
            operator,
            value,
        }
    }

    pub fn column(&self) -> &ColumnName {
        &self.column
    }

    pub fn operator(&self) -> Operator {
        self.operator
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}