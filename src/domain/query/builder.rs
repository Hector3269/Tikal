use crate::domain::model::{Entity, ModelMapping};
use crate::domain::value_objects::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
    Like,
    In,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub column: String,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub values: Vec<Value>,
}

pub struct QueryBuilder<E: Entity> {
    pub table_name: String,
    pub filters: Vec<Condition>,
    pub order_by: Vec<OrderBy>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub with_relations: Vec<String>,
    _entity: PhantomData<E>,
}

impl<E: Entity> QueryBuilder<E> {
    pub fn new() -> Self {
        Self {
            table_name: E::table_name().to_string(),
            filters: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            with_relations: Vec::new(),
            _entity: PhantomData,
        }
    }

    pub fn where_clause(
        mut self,
        column: &str,
        operator: Operator,
        value: impl Into<Value>,
    ) -> Self {
        self.filters.push(Condition {
            column: column.to_string(),
            operator,
            values: vec![value.into()],
        });
        self
    }

    pub fn where_in(mut self, column: &str, values: Vec<impl Into<Value>>) -> Self {
        self.filters.push(Condition {
            column: column.to_string(),
            operator: Operator::In,
            values: values.into_iter().map(|v| v.into()).collect(),
        });
        self
    }

    pub fn where_field(mut self, field: &str, operator: Operator, value: impl Into<Value>) -> Self
    where
        E: ModelMapping,
    {
        let column = E::field_to_column(field).unwrap_or_else(|| field.to_string());
        self.filters.push(Condition {
            column,
            operator,
            values: vec![value.into()],
        });
        self
    }

    pub fn where_field_in(mut self, field: &str, values: Vec<impl Into<Value>>) -> Self
    where
        E: ModelMapping,
    {
        let column = E::field_to_column(field).unwrap_or_else(|| field.to_string());
        self.filters.push(Condition {
            column,
            operator: Operator::In,
            values: values.into_iter().map(|v| v.into()).collect(),
        });
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with(mut self, relation: &str) -> Self {
        self.with_relations.push(relation.to_string());
        self
    }

    pub fn order_by(mut self, column: &str, direction: OrderDirection) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            direction,
        });
        self
    }

    pub fn order_by_asc(mut self, column: &str) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            direction: OrderDirection::Asc,
        });
        self
    }

    pub fn order_by_desc(mut self, column: &str) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            direction: OrderDirection::Desc,
        });
        self
    }

    pub fn order_by_field(mut self, field: &str, direction: OrderDirection) -> Self
    where
        E: crate::domain::model::ModelMapping,
    {
        let column = E::field_to_column(field).unwrap_or_else(|| field.to_string());
        self.order_by.push(OrderBy { column, direction });
        self
    }

    pub fn order_by_field_asc(self, field: &str) -> Self
    where
        E: crate::domain::model::ModelMapping,
    {
        self.order_by_field(field, OrderDirection::Asc)
    }

    pub fn order_by_field_desc(self, field: &str) -> Self
    where
        E: crate::domain::model::ModelMapping,
    {
        self.order_by_field(field, OrderDirection::Desc)
    }

    pub async fn all<R>(self, repo: &R) -> crate::domain::TikalResult<Vec<E>>
    where
        R: crate::domain::repositories::Repository<E>,
    {
        repo.find_with_query(self).await
    }

    pub async fn first<R>(self, repo: &R) -> crate::domain::TikalResult<Option<E>>
    where
        R: crate::domain::repositories::Repository<E>,
    {
        repo.find_first_with_query(self).await
    }
}
