use crate::domain::repositories::executor::QueryExecutor as DomainQueryExecutor;
use crate::domain::repositories::executor::Transaction as DomainTransaction;
use crate::domain::repositories::types::DriverInfo;
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct DatabaseExecutor<DB, B, M>
where
    DB: sqlx::Database,
    B: for<'q> super::ParameterBinder<'q, DB> + Send + Sync,
    M: super::RowMapper<DB> + Send + Sync,
{
    pub(crate) pool: sqlx::Pool<DB>,
    pub(crate) driver_info: DriverInfo,
    pub(crate) _binder: std::marker::PhantomData<B>,
    pub(crate) _mapper: std::marker::PhantomData<M>,
}

#[async_trait]
impl<DB, B, M> DomainQueryExecutor for DatabaseExecutor<DB, B, M>
where
    DB: sqlx::Database,
    B: for<'q> super::ParameterBinder<'q, DB> + Send + Sync + 'static,
    M: super::RowMapper<DB> + Send + Sync + 'static,
    for<'r> &'r str: sqlx::ColumnIndex<DB::Row>,
    for<'q> <DB as sqlx::Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
    for<'c> &'c mut <DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = DB>,
    for<'c> &'c sqlx::Pool<DB>: sqlx::Executor<'c, Database = DB>,
{
    async fn fetch_all(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        let query = sqlx::query(sql);
        let query = B::bind_params(query, params);
        let rows = query.fetch_all(&self.pool).await?;
        M::map_rows(rows)
    }

    async fn fetch_one(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<HashMap<String, Value>> {
        let results = self.fetch_all(sql, params).await?;
        results.into_iter().next().ok_or_else(|| {
            crate::domain::TikalError::database_error(
                "No rows returned",
                "Expected exactly one row but got none",
                None,
            )
        })
    }

    async fn fetch_optional(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>> {
        let results = self.fetch_all(sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        let query = sqlx::query(sql);
        let query = B::bind_params(query, params);
        query.execute(&self.pool).await?;
        Ok(1)
    }

    async fn execute_with_rows(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        self.execute(sql, params).await
    }

    async fn begin_transaction(&self) -> TikalResult<Box<dyn DomainTransaction>> {
        let tx = self.pool.begin().await?;
        Ok(Box::new(super::DatabaseTransaction::<DB, B, M>::new(tx)))
    }

    async fn ping(&self) -> TikalResult<bool> {
        Ok(self.fetch_all("SELECT 1", vec![]).await.is_ok())
    }

    fn driver_info(&self) -> DriverInfo {
        self.driver_info.clone()
    }
}
