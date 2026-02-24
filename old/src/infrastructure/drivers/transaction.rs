use crate::domain::repositories::executor::Transaction as DomainTransaction;
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct DatabaseTransaction<DB, B, M>
where
    DB: sqlx::Database,
    B: for<'q> super::ParameterBinder<'q, DB> + Send + Sync,
    M: super::RowMapper<DB> + Send + Sync,
{
    tx: Option<sqlx::Transaction<'static, DB>>,
    _binder: std::marker::PhantomData<B>,
    _mapper: std::marker::PhantomData<M>,
}

impl<DB, B, M> DatabaseTransaction<DB, B, M>
where
    DB: sqlx::Database,
    B: for<'q> super::ParameterBinder<'q, DB> + Send + Sync,
    M: super::RowMapper<DB> + Send + Sync,
{
    pub fn new(tx: sqlx::Transaction<'static, DB>) -> Self {
        Self {
            tx: Some(tx),
            _binder: std::marker::PhantomData,
            _mapper: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<DB, B, M> DomainTransaction for DatabaseTransaction<DB, B, M>
where
    DB: sqlx::Database,
    B: for<'q> super::ParameterBinder<'q, DB> + Send + Sync,
    M: super::RowMapper<DB> + Send + Sync,
    for<'r> &'r str: sqlx::ColumnIndex<DB::Row>,
    for<'q> <DB as sqlx::Database>::Arguments<'q>: sqlx::IntoArguments<'q, DB>,
    for<'c> &'c mut <DB as sqlx::Database>::Connection: sqlx::Executor<'c, Database = DB>,
{
    async fn fetch_all(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<HashMap<String, Value>>> {
        let tx = self.tx.as_mut().ok_or_else(|| {
            crate::domain::TikalError::database_error(
                "Transaction already consumed",
                "Cannot execute query on consumed transaction",
                None,
            )
        })?;

        let query = sqlx::query(sql);
        let query = B::bind_params(query, params);
        let rows = query.fetch_all(&mut **tx).await?;
        M::map_rows(rows)
    }

    async fn fetch_one(
        &mut self,
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
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Option<HashMap<String, Value>>> {
        let results = self.fetch_all(sql, params).await?;
        Ok(results.into_iter().next())
    }

    async fn execute(&mut self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        let tx = self.tx.as_mut().ok_or_else(|| {
            crate::domain::TikalError::database_error(
                "Transaction already consumed",
                "Cannot execute query on consumed transaction",
                None,
            )
        })?;

        let query = sqlx::query(sql);
        let query = B::bind_params(query, params);
        query.execute(&mut **tx).await?;
        Ok(1)
    }

    async fn commit(mut self: Box<Self>) -> TikalResult<()> {
        let tx = self.tx.take().ok_or_else(|| {
            crate::domain::TikalError::database_error(
                "Transaction already consumed",
                "Cannot commit consumed transaction",
                None,
            )
        })?;
        tx.commit().await?;
        Ok(())
    }

    async fn rollback(mut self: Box<Self>) -> TikalResult<()> {
        let tx = self.tx.take().ok_or_else(|| {
            crate::domain::TikalError::database_error(
                "Transaction already consumed",
                "Cannot rollback consumed transaction",
                None,
            )
        })?;
        tx.rollback().await?;
        Ok(())
    }
}
