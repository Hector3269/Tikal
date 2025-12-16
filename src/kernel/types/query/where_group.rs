use super::where_clause::WhereClause;

/// Represents a group of WHERE conditions combined with AND or OR logic.
#[derive(Debug, Clone, PartialEq)]
pub struct WhereGroup {
    conditions: Vec<WhereClause>,
    logic: LogicOperator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicOperator {
    And,
    Or,
}

impl WhereGroup {
    pub fn new(conditions: Vec<WhereClause>, logic: LogicOperator) -> Self {
        Self { conditions, logic }
    }

    pub fn conditions(&self) -> &[WhereClause] {
        &self.conditions
    }

    pub fn logic(&self) -> LogicOperator {
        self.logic
    }
}