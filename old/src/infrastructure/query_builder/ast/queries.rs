use super::expressions::{Condition, Expression, Join, OrderBy};

#[derive(Debug, Clone)]
pub struct SelectQuery {
    pub table: String,
    pub columns: Vec<Expression>,
    pub distinct: bool,
    pub joins: Vec<Join>,
    pub filters: Vec<Condition>,
    pub group_by: Vec<Expression>,
    pub having: Vec<Condition>,
    pub order_by: Vec<OrderBy>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct InsertQuery {
    pub table: String,
    pub columns: Vec<String>,
    pub values: Vec<crate::domain::value_objects::Value>,
}

#[derive(Debug, Clone)]
pub struct UpdateQuery {
    pub table: String,
    pub assignments: Vec<(String, crate::domain::value_objects::Value)>,
    pub filters: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub struct DeleteQuery {
    pub table: String,
    pub filters: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub enum Query {
    Select(SelectQuery),
    Insert(InsertQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
}
