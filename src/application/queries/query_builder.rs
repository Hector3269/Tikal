use std::marker::PhantomData;
use async_trait::async_trait;
use crate::kernel::types::query::{WhereClause, Operator, OrderBy, SortDirection, Limit, Offset};
use crate::kernel::types::schema::ColumnName;
use crate::kernel::types::core::Value;
use crate::domain::{Entity, Model};
use crate::kernel::error::KernelError;

/// Generic query builder for entities
pub struct QueryBuilder<T> {
    table: String,
    wheres: Vec<WhereClause>,
    order_by: Option<OrderBy>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    with_relations: Vec<String>,
    _phantom: PhantomData<T>,
}

impl<T> QueryBuilder<T> {
    pub fn new(table: String) -> Self {
        Self {
            table,
            wheres: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
            with_relations: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn where_clause(mut self, column: &str, operator: Operator, value: impl Into<Value>) -> Self {
        let column_name = ColumnName::new(column);
        self.wheres.push(WhereClause::new(column_name, operator, value.into()));
        self
    }

    pub fn where_eq(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_clause(column, Operator::Eq, value)
    }

    pub fn order_by(mut self, column: &str, direction: SortDirection) -> Self {
        let column_name = ColumnName::new(column);
        self.order_by = Some(OrderBy::new(column_name, direction));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(Limit::new(limit as u32).unwrap());
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(Offset::new(offset as u32));
        self
    }

    pub fn with(mut self, relations: &[&str]) -> Self {
        self.with_relations.extend(relations.iter().map(|s| s.to_string()));
        self
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("SELECT * FROM {}", self.table);

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            let where_strs: Vec<String> = self.wheres.iter()
                .map(|w| {
                    if *w.value() == Value::Null {
                        format!("{} IS NULL", w.column().as_str())
                    } else {
                        format!("{} {} ?", w.column().as_str(), w.operator().as_str())
                    }
                })
                .collect();
            sql.push_str(&where_strs.join(" AND "));
        }

        if let Some(order_by) = &self.order_by {
            sql.push_str(&format!(" ORDER BY {} {}", order_by.column().as_str(), order_by.direction().as_str()));
        }

        if let Some(limit) = &self.limit {
            sql.push_str(&format!(" LIMIT {}", limit.value()));
        }

        if let Some(offset) = &self.offset {
            sql.push_str(&format!(" OFFSET {}", offset.value()));
        }

        sql
    }

    pub fn params(&self) -> Vec<&Value> {
        self.wheres.iter()
            .filter(|w| *w.value() != Value::Null)
            .map(|w| w.value())
            .collect()
    }
}

impl<T> Default for QueryBuilder<T> {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl<T> crate::domain::Scopes for QueryBuilder<T>
where
    T: Model,
{
    fn scope_active(self) -> Self {
        self.where_eq("active", true)
    }

    fn scope_published(self) -> Self {
        self.where_eq("published", true)
    }
}

#[async_trait(?Send)]
pub trait Queryable<T: Entity> {
    async fn get<E>(query: QueryBuilder<T>, executor: &E) -> Result<Vec<T>, KernelError>
    where
        E: crate::infrastructure::database::executor::QueryExecutor;
    async fn first<E>(query: QueryBuilder<T>, executor: &E) -> Result<Option<T>, KernelError>
    where
        E: crate::infrastructure::database::executor::QueryExecutor;
    async fn count<E>(query: QueryBuilder<T>, executor: &E) -> Result<i64, KernelError>
    where
        E: crate::infrastructure::database::executor::QueryExecutor;
}

pub struct BasicQueryable;

#[async_trait(?Send)]
impl<T> Queryable<T> for BasicQueryable
where
    T: Entity + From<crate::kernel::types::db::DbRow> + crate::domain::SoftDeletes + Send + Sync + 'static,
{
    async fn get<E>(mut query: QueryBuilder<T>, executor: &E) -> Result<Vec<T>, KernelError>
    where
        E: crate::infrastructure::database::executor::QueryExecutor,
    {
        if T::uses_soft_deletes() {
            query = query.where_clause("deleted_at", crate::kernel::types::query::Operator::Eq, Value::Null);
        }

        let sql = query.to_sql();
        let params = query.params().into_iter().map(|v| v.clone()).collect::<Vec<_>>();
        let rows = executor.query_raw(&sql, &params).await.map_err(|e| KernelError::DatabaseError { source: format!("Query error: {}", e) })?;
        Ok(rows.into_iter().map(T::from).collect())
    }

    async fn first<E>(query: QueryBuilder<T>, executor: &E) -> Result<Option<T>, KernelError>
    where
        E: crate::infrastructure::database::executor::QueryExecutor,
    {
        let limited_query = query.limit(1);
        let results: Vec<T> = Self::get(limited_query, executor).await?;
        Ok(results.into_iter().next())
    }

    async fn count<E>(query: QueryBuilder<T>, executor: &E) -> Result<i64, KernelError>
    where
        E: crate::infrastructure::database::executor::QueryExecutor,
    {
        let sql = format!("SELECT COUNT(*) FROM ({}) AS subquery", query.to_sql());
        let params = query.params().into_iter().map(|v| v.clone()).collect::<Vec<_>>();
        let rows = executor.query_raw(&sql, &params).await.map_err(|e| KernelError::DatabaseError { source: format!("Count query error: {}", e) })?;
        if let Some(row) = rows.into_iter().next() {
            if let Some(crate::kernel::types::core::Value::Int(count)) = row.get("COUNT(*)") {
                Ok(*count)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }
}