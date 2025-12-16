use crate::kernel::types::schema::table_name::TableName;
use crate::kernel::types::query::join_type::JoinType;
use crate::kernel::types::schema::column_name::ColumnName;
use crate::kernel::types::query::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub struct JoinClause {
    table: TableName,
    join_type: JoinType,
    on_conditions: Vec<JoinOn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinOn {
    left_column: ColumnName,
    operator: Operator,
    right_column: ColumnName,
}

impl JoinClause {
    pub fn new(table: TableName, join_type: JoinType, on_conditions: Vec<JoinOn>) -> Self {
        Self {
            table,
            join_type,
            on_conditions,
        }
    }

    pub fn table(&self) -> &TableName {
        &self.table
    }

    pub fn join_type(&self) -> JoinType {
        self.join_type
    }

    pub fn on_conditions(&self) -> &[JoinOn] {
        &self.on_conditions
    }
}

impl JoinOn {
    pub fn new(left_column: ColumnName, operator: Operator, right_column: ColumnName) -> Self {
        Self {
            left_column,
            operator,
            right_column,
        }
    }

    pub fn left_column(&self) -> &ColumnName {
        &self.left_column
    }

    pub fn operator(&self) -> Operator {
        self.operator
    }

    pub fn right_column(&self) -> &ColumnName {
        &self.right_column
    }
}