use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::value_objects::Value;
pub trait QueryGenerator: Send + Sync {
    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>);
    fn generate_insert<E: Entity>(&self, entity: &E) -> (String, Vec<Value>);
    fn generate_update<E: Entity>(&self, entity: &E) -> (String, Vec<Value>);
    fn generate_delete<E: Entity>(&self, entity: &E) -> (String, Vec<Value>);
    fn generate_count<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>);
    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>);
    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>);
    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>);
    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>);
}
