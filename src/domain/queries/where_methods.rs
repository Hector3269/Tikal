use super::query_builder::QueryBuilder;
use super::{Operator, WhereCondition};
use crate::infrastructure::core::types::Value;

impl<T> QueryBuilder<T> {
    pub fn where_(mut self, column: &str, operator: Operator, value: impl Into<Value>) -> Self {
        self.wheres
            .push(WhereCondition::simple(column, operator, value.into()));
        self
    }

    pub fn where_eq(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_(column, Operator::Eq, value)
    }

    pub fn where_ne(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_(column, Operator::Ne, value)
    }

    pub fn where_gt(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_(column, Operator::Gt, value)
    }

    pub fn where_gte(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_(column, Operator::Gte, value)
    }

    pub fn where_lt(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_(column, Operator::Lt, value)
    }

    pub fn where_lte(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_(column, Operator::Lte, value)
    }

    pub fn where_like(self, column: &str, pattern: &str) -> Self {
        self.where_(column, Operator::Like, pattern)
    }

    pub fn where_null(mut self, column: &str) -> Self {
        self.wheres
            .push(WhereCondition::null(column, Operator::IsNull));
        self
    }

    pub fn where_not_null(mut self, column: &str) -> Self {
        self.wheres
            .push(WhereCondition::null(column, Operator::IsNotNull));
        self
    }

    pub fn where_in(mut self, column: &str, values: Vec<impl Into<Value>>) -> Self {
        let values = values.into_iter().map(|v| v.into()).collect();
        self.wheres
            .push(WhereCondition::multi(column, Operator::In, values));
        self
    }

    pub fn where_not_in(mut self, column: &str, values: Vec<impl Into<Value>>) -> Self {
        let values = values.into_iter().map(|v| v.into()).collect();
        self.wheres
            .push(WhereCondition::multi(column, Operator::NotIn, values));
        self
    }
}
