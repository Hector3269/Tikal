use crate::domain::query::builder::{Operator, OrderDirection};
use crate::domain::value_objects::Value;

#[derive(Debug, Clone)]
pub enum Expression {
    Column(String),
    QualifiedColumn(String, String),
    Literal(Value),
    Function(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub left: Expression,
    pub operator: Operator,
    pub right: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub expression: Expression,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, Clone)]
pub struct Join {
    pub table: String,
    pub on: Condition,
    pub join_type: JoinType,
}
