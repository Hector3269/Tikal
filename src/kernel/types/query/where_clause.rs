use crate::kernel::types::schema::column_name::ColumnName;
use crate::kernel::types::query::operator::Operator;
use crate::kernel::types::core::value::Value;

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