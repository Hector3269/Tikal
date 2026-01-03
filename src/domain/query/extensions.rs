use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::TikalResult;

pub trait QueryBuilderExt<E: Entity> {
    fn where_<F>(self, filter_fn: F) -> QueryBuilder<E>
    where
        F: FnOnce() -> crate::domain::query::filter::Filter;

    fn where_all<F>(self, filter_fn: F) -> QueryBuilder<E>
    where
        F: FnOnce() -> crate::domain::query::filter::FilterGroup;

    fn order_by_<F>(self, order_fn: F) -> QueryBuilder<E>
    where
        F: FnOnce() -> crate::domain::query::order::OrderGroup;
}

impl<E: Entity> QueryBuilderExt<E> for QueryBuilder<E> {
    fn where_<F>(self, filter_fn: F) -> QueryBuilder<E>
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
    {
        let filter = filter_fn();
        if !filter.values.is_empty() {
            self.where_clause(
                &filter.column,
                filter.operator,
                filter.values.clone().into_iter().next().unwrap(),
            )
        } else {
            self
        }
    }

    fn where_all<F>(self, filter_fn: F) -> QueryBuilder<E>
    where
        F: FnOnce() -> crate::domain::query::filter::FilterGroup,
    {
        let filter_group = filter_fn();
        let mut result = self;

        for filter in filter_group.iter() {
            if !filter.values.is_empty() {
                result = result.where_clause(
                    &filter.column,
                    filter.operator,
                    filter.values.clone().into_iter().next().unwrap(),
                );
            }
        }

        result
    }

    fn order_by_<F>(self, order_fn: F) -> QueryBuilder<E>
    where
        F: FnOnce() -> crate::domain::query::order::OrderGroup,
    {
        let order_group = order_fn();
        let mut result = self;

        for order in order_group.iter() {
            result = result.order_by(&order.column, order.direction);
        }

        result
    }
}

pub trait RepositoryExt<E: Entity, R> {
    fn find_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<Vec<E>>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
        R: crate::domain::repositories::Repository<E>;

    fn find_first_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<Option<E>>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
        R: crate::domain::repositories::Repository<E>;

    fn find_ordered<F>(
        &self,
        order_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<Vec<E>>> + Send
    where
        F: FnOnce() -> crate::domain::query::order::OrderGroup,
        R: crate::domain::repositories::Repository<E>;

    fn find_paginated(
        &self,
        page: usize,
        per_page: usize,
    ) -> impl std::future::Future<Output = TikalResult<Vec<E>>> + Send
    where
        R: crate::domain::repositories::Repository<E>;

    fn count_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<i64>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
        R: crate::domain::repositories::Repository<E>;

    fn exists_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<bool>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
        R: crate::domain::repositories::Repository<E>;
}

impl<E: Entity, R> RepositoryExt<E, R> for R
where
    R: crate::domain::repositories::Repository<E>,
{
    fn find_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<Vec<E>>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
    {
        let filter = filter_fn();
        async move {
            if !filter.values.is_empty() {
                let query = E::find().where_clause(
                    &filter.column,
                    filter.operator,
                    filter.values.clone().into_iter().next().unwrap(),
                );
                self.find_with_query(query).await
            } else {
                Ok(Vec::new())
            }
        }
    }

    fn find_first_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<Option<E>>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
    {
        let filter = filter_fn();
        async move {
            if !filter.values.is_empty() {
                let query = E::find().where_clause(
                    &filter.column,
                    filter.operator,
                    filter.values.clone().into_iter().next().unwrap(),
                );
                self.find_first_with_query(query).await
            } else {
                Ok(None)
            }
        }
    }

    fn find_ordered<F>(
        &self,
        order_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<Vec<E>>> + Send
    where
        F: FnOnce() -> crate::domain::query::order::OrderGroup,
    {
        let order_group = order_fn();
        let mut query = E::find();

        for order in order_group.iter() {
            query = query.order_by(&order.column, order.direction);
        }

        async move { self.find_with_query(query).await }
    }

    fn find_paginated(
        &self,
        _page: usize,
        per_page: usize,
    ) -> impl std::future::Future<Output = TikalResult<Vec<E>>> + Send {
        async move {
            let query = E::find().limit(per_page);
            self.find_with_query(query).await
        }
    }

    fn count_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<i64>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
    {
        let filter = filter_fn();
        async move {
            if !filter.values.is_empty() {
                let query = E::find().where_clause(
                    &filter.column,
                    filter.operator,
                    filter.values.clone().into_iter().next().unwrap(),
                );
                self.count(query).await
            } else {
                Ok(0)
            }
        }
    }

    fn exists_where<F>(
        &self,
        filter_fn: F,
    ) -> impl std::future::Future<Output = TikalResult<bool>> + Send
    where
        F: FnOnce() -> crate::domain::query::filter::Filter,
    {
        let filter = filter_fn();
        async move {
            if !filter.values.is_empty() {
                let query = E::find()
                    .where_clause(
                        &filter.column,
                        filter.operator,
                        filter.values.clone().into_iter().next().unwrap(),
                    )
                    .limit(1);
                let result = self.find_first_with_query(query).await?;
                Ok(result.is_some())
            } else {
                Ok(false)
            }
        }
    }
}
