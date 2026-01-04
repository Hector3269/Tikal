use crate::domain::model::Entity;
use crate::domain::query::builder::{Operator, QueryBuilder};
use crate::domain::repositories::{executor::QueryExecutor, Repository};
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use crate::infrastructure::query_builder::generators::{SqlGenerator, SqlGeneratorEnum};
use async_trait::async_trait;
use std::marker::PhantomData;

pub struct SqlRepository<E, EX>
where
    E: Entity,
    EX: QueryExecutor + Sync + Send,
{
    pub generator: SqlGeneratorEnum,
    pub executor: EX,
    pub _phantom: PhantomData<E>,
}

impl<E, EX> SqlRepository<E, EX>
where
    E: Entity,
    EX: QueryExecutor + Sync + Send,
{
    pub fn new(generator: SqlGeneratorEnum, executor: EX) -> Self {
        Self {
            generator,
            executor,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<E, EX> Repository<E> for SqlRepository<E, EX>
where
    E: Entity,
    EX: QueryExecutor + Sync + Send,
{
    async fn find_by_id(&self, id: &Value) -> TikalResult<Option<E>> {
        let builder = E::find().where_clause(E::primary_key(), Operator::Eq, id.clone());
        let (sql, params) = self.generator.generate_select(&builder);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if rows.is_empty() {
            Ok(None)
        } else {
            let entity = E::from_row(rows.into_iter().next().unwrap())?;
            Ok(Some(entity))
        }
    }

    async fn find_all(&self) -> TikalResult<Vec<E>> {
        let builder = E::find();
        self.find_with_query(builder).await
    }

    async fn find_with_query(&self, query: QueryBuilder<E>) -> TikalResult<Vec<E>> {
        let (sql, params) = self.generator.generate_select(&query);
        let rows = self.executor.fetch_all(&sql, params).await?;
        let mut entities = Vec::new();
        for row in rows {
            entities.push(E::from_row(row)?);
        }
        for relation in &query.with_relations {
            E::eager_load(
                &mut entities,
                relation,
                &self.executor as &dyn QueryExecutor,
                &self.generator,
            )
            .await?;
        }
        Ok(entities)
    }

    async fn find_first_with_query(&self, query: QueryBuilder<E>) -> TikalResult<Option<E>> {
        let query = query.limit(1);
        let (sql, params) = self.generator.generate_select(&query);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let entity = E::from_row(row)?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, entity: &E) -> TikalResult<u64> {
        let (sql, params) = self.generator.generate_insert(entity);
        self.executor.execute(&sql, params).await.map_err(|e| {
            e.with_context(format!(
                "Failed to save entity to table '{}'",
                E::table_name()
            ))
        })
    }

    async fn update(&self, entity: &E) -> TikalResult<u64> {
        let values = entity.to_values();
        let pk = E::primary_key();
        if !values.contains_key(pk)
            || matches!(values[pk], crate::domain::value_objects::Value::Null)
        {
            return Err(crate::domain::error::TikalError::validation(
                pk,
                &format!("Primary key '{}' must be set for update operation", pk),
            ));
        }

        let (sql, params) = self.generator.generate_update(entity);
        self.executor.execute(&sql, params).await.map_err(|e| {
            e.with_context(format!(
                "Failed to update entity in table '{}' with primary key '{}'",
                E::table_name(),
                pk
            ))
        })
    }

    async fn delete(&self, entity: &E) -> TikalResult<u64> {
        let values = entity.to_values();
        let pk = E::primary_key();
        if !values.contains_key(pk)
            || matches!(values[pk], crate::domain::value_objects::Value::Null)
        {
            return Err(crate::domain::error::TikalError::validation(
                pk,
                &format!("Primary key '{}' must be set for delete operation", pk),
            ));
        }

        let (sql, params) = self.generator.generate_delete(entity);
        self.executor.execute(&sql, params).await.map_err(|e| {
            e.with_context(format!(
                "Failed to delete entity from table '{}' with primary key '{}'",
                E::table_name(),
                pk
            ))
        })
    }

    async fn count(&self, query: QueryBuilder<E>) -> TikalResult<i64> {
        let (sql, params) = self.generator.generate_count(&query);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let count_value = row
                .get("count")
                .or_else(|| row.get("COUNT(*)"))
                .ok_or_else(|| {
                    crate::domain::error::TikalError::mapping("count", "missing count column")
                })?;
            match count_value {
                crate::domain::value_objects::Value::Int(i) => Ok(*i),
                _ => Err(crate::domain::error::TikalError::mapping(
                    "count",
                    "count is not int",
                )),
            }
        } else {
            Ok(0)
        }
    }

    async fn sum(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<f64>> {
        let (sql, params) = self.generator.generate_sum(&query, field);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let sum_value = row
                .get("sum")
                .or_else(|| row.get(&format!("SUM({})", field)))
                .ok_or_else(|| {
                    crate::domain::error::TikalError::mapping("sum", "missing sum column")
                })?;
            match sum_value {
                crate::domain::value_objects::Value::Float(f) => Ok(Some(f.into_inner())),
                crate::domain::value_objects::Value::Int(i) => Ok(Some(*i as f64)),
                crate::domain::value_objects::Value::Null => Ok(None),
                _ => Err(crate::domain::error::TikalError::mapping(
                    "sum",
                    "sum is not numeric",
                )),
            }
        } else {
            Ok(None)
        }
    }

    async fn avg(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<f64>> {
        let (sql, params) = self.generator.generate_avg(&query, field);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let avg_value = row
                .get("avg")
                .or_else(|| row.get(&format!("AVG({})", field)))
                .ok_or_else(|| {
                    crate::domain::error::TikalError::mapping("avg", "missing avg column")
                })?;
            match avg_value {
                crate::domain::value_objects::Value::Float(f) => Ok(Some(f.into_inner())),
                crate::domain::value_objects::Value::Int(i) => Ok(Some(*i as f64)),
                crate::domain::value_objects::Value::Null => Ok(None),
                _ => Err(crate::domain::error::TikalError::mapping(
                    "avg",
                    "avg is not numeric",
                )),
            }
        } else {
            Ok(None)
        }
    }

    async fn min(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<Value>> {
        let (sql, params) = self.generator.generate_min(&query, field);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let min_value = row
                .get("min")
                .or_else(|| row.get(&format!("MIN({})", field)))
                .ok_or_else(|| {
                    crate::domain::error::TikalError::mapping("min", "missing min column")
                })?;
            Ok(Some(min_value.clone()))
        } else {
            Ok(None)
        }
    }

    async fn max(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<Value>> {
        let (sql, params) = self.generator.generate_max(&query, field);
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let max_value = row
                .get("max")
                .or_else(|| row.get(&format!("MAX({})", field)))
                .ok_or_else(|| {
                    crate::domain::error::TikalError::mapping("max", "missing max column")
                })?;
            Ok(Some(max_value.clone()))
        } else {
            Ok(None)
        }
    }

    async fn save_many(&self, entities: &[E]) -> TikalResult<u64> {
        let mut total = 0;
        for entity in entities {
            total += self.save(entity).await?;
        }
        Ok(total)
    }

    async fn update_many(&self, entities: &[E]) -> TikalResult<u64> {
        let mut total = 0;
        for entity in entities {
            total += self.update(entity).await?;
        }
        Ok(total)
    }

    async fn delete_many(&self, entities: &[E]) -> TikalResult<u64> {
        let mut total = 0;
        for entity in entities {
            total += self.delete(entity).await?;
        }
        Ok(total)
    }

    async fn execute_raw(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64> {
        self.executor.execute(sql, params).await
    }

    async fn query_raw(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<std::collections::HashMap<String, Value>>> {
        self.executor.fetch_all(sql, params).await
    }
}
