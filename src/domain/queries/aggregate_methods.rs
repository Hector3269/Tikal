use super::query_builder::QueryBuilder;
use crate::domain::QueryExecutor;
use crate::infrastructure::core::types::Value;
use crate::TikalError;

impl<T> QueryBuilder<T>
where
    T: crate::domain::Model + Send + Sync + 'static,
{
    pub async fn count<E>(self, executor: &E) -> Result<i64, TikalError>
    where
        E: QueryExecutor,
    {
        let sql = format!("SELECT COUNT(*) as count FROM ({}) t", self.to_sql());
        let params = self.to_params();

        let rows = executor.query_raw(&sql, &params).await?;
        let row = rows
            .first()
            .ok_or_else(|| TikalError::not_implemented("No rows"))?;

        Ok(row
            .get("count")
            .and_then(|v| match v {
                Value::Int(i) => Some(*i),
                _ => None,
            })
            .unwrap_or(0))
    }

    pub async fn exists<E>(self, executor: &E) -> Result<bool, TikalError>
    where
        E: QueryExecutor,
    {
        Ok(self.limit(1).count(executor).await? > 0)
    }
}
