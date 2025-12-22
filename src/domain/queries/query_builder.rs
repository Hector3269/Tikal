use super::{Operator, WhereCondition};
use crate::domain::*;
use crate::infrastructure::database::executor::QueryExecutor;
use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct QueryBuilder<T> {
    _phantom: PhantomData<T>,
    pub wheres: Vec<WhereCondition>,
    pub orders: Vec<(String, String)>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub select: Option<Vec<String>>,
}

impl<T> QueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            wheres: Vec::new(),
            orders: Vec::new(),
            limit: None,
            offset: None,
            select: None,
        }
    }
}

impl<T> QueryBuilder<T>
where
    T: Model + Send + Sync + 'static,
{
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn select(mut self, columns: Vec<&str>) -> Self {
        self.select = Some(columns.into_iter().map(|s| s.to_string()).collect());
        self
    }

    pub async fn first<E>(self, executor: &E) -> Result<Option<T>, KernelError>
    where
        E: QueryExecutor,
    {
        Ok(self.limit(1).get(executor).await?.into_iter().next())
    }

    pub async fn get<E>(self, executor: &E) -> Result<Vec<T>, KernelError>
    where
        E: QueryExecutor,
    {
        let rows = executor
            .query_raw(&self.to_sql(), &self.to_params())
            .await?;
        rows.into_iter()
            .map(<T as crate::domain::ActiveRecord>::from_row)
            .collect()
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!(
            "SELECT {} FROM {}",
            self.select
                .as_ref()
                .map(|c| c.join(", "))
                .unwrap_or_else(|| "*".to_string()),
            T::table_name()
        );

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(
                &self
                    .wheres
                    .iter()
                    .map(|w| match (&w.operator, &w.values) {
                        (Operator::In | Operator::NotIn, Some(values)) => {
                            let p = vec!["?"; values.len()].join(", ");
                            format!("{} {} ({})", w.column, w.operator.as_sql(), p)
                        }
                        (Operator::IsNull | Operator::IsNotNull, _) => {
                            format!("{} {}", w.column, w.operator.as_sql())
                        }
                        _ => format!("{} {} ?", w.column, w.operator.as_sql()),
                    })
                    .collect::<Vec<_>>()
                    .join(" AND "),
            );
        }

        if !self.orders.is_empty() {
            sql.push_str(" ORDER BY ");
            sql.push_str(
                &self
                    .orders
                    .iter()
                    .map(|(c, d)| format!("{} {}", c, d))
                    .collect::<Vec<_>>()
                    .join(", "),
            );
        }

        if let Some(l) = self.limit {
            sql.push_str(&format!(" LIMIT {}", l));
        }

        if let Some(o) = self.offset {
            sql.push_str(&format!(" OFFSET {}", o));
        }

        sql
    }

    pub fn to_params(&self) -> Vec<Value> {
        let mut params = Vec::new();
        for w in &self.wheres {
            if let Some(v) = &w.value {
                params.push(v.clone());
            }
            if let Some(vs) = &w.values {
                params.extend(vs.clone());
            }
        }
        params
    }
}

pub trait Queryable: Model + FromRow + Sized {
    fn query() -> QueryBuilder<Self> {
        QueryBuilder::new()
    }
}

impl<T> Queryable for T where T: Model + FromRow + Sized {}
