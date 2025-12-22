use super::query_builder::QueryBuilder;
use crate::domain::FromRow;

pub trait Scopable: Sized {
    fn scope<F>(query: QueryBuilder<Self>, scope_fn: F) -> QueryBuilder<Self>
    where
        F: FnOnce(QueryBuilder<Self>) -> QueryBuilder<Self>,
    {
        scope_fn(query)
    }
}

impl<T> Scopable for T where T: crate::domain::Model + FromRow + Sized {}
