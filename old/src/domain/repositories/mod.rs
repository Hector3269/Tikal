pub mod executor;
pub mod types;

use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use async_trait::async_trait;

#[async_trait]
pub trait Repository<E: Entity>: Send + Sync {
    async fn find_by_id(&self, id: &Value) -> TikalResult<Option<E>>;
    async fn find_all(&self) -> TikalResult<Vec<E>>;
    async fn find_with_query(&self, query: QueryBuilder<E>) -> TikalResult<Vec<E>>;
    async fn find_first_with_query(&self, query: QueryBuilder<E>) -> TikalResult<Option<E>>;
    async fn count(&self, query: QueryBuilder<E>) -> TikalResult<i64>;
    async fn sum(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<f64>>;
    async fn avg(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<f64>>;
    async fn min(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<Value>>;
    async fn max(&self, query: QueryBuilder<E>, field: &str) -> TikalResult<Option<Value>>;
    async fn save(&self, entity: &E) -> TikalResult<u64>;
    async fn save_many(&self, entities: &[E]) -> TikalResult<u64>;
    async fn update(&self, entity: &E) -> TikalResult<u64>;
    async fn update_many(&self, entities: &[E]) -> TikalResult<u64>;
    async fn delete(&self, entity: &E) -> TikalResult<u64>;
    async fn delete_many(&self, entities: &[E]) -> TikalResult<u64>;
    async fn execute_raw(&self, sql: &str, params: Vec<Value>) -> TikalResult<u64>;
    async fn query_raw(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> TikalResult<Vec<std::collections::HashMap<String, Value>>>;
}
