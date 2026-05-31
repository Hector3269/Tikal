use crate::domain::query::builder::QueryBuilder;
use crate::domain::query_generator::QueryGenerator;
use crate::domain::repositories::executor::QueryExecutor;
use crate::domain::TikalResult;
use crate::infrastructure::schema::types::TableDefinition;
use std::collections::HashMap;

pub trait ModelMapping {
    fn column_mappings() -> HashMap<String, String>;

    fn field_to_column(field_name: &str) -> Option<String>;

    fn column_to_field(column_name: &str) -> Option<String>;
}

pub trait FromRow: Sized {
    fn from_row(row: HashMap<String, crate::domain::value_objects::Value>) -> TikalResult<Self>;
}

pub trait Entity: Sized + FromRow + Send + Sync {
    fn table_name() -> &'static str;

    fn to_values(&self) -> HashMap<String, crate::domain::value_objects::Value>;

    fn relationships() -> crate::domain::model::relationships::RelationshipMap {
        HashMap::new()
    }

    fn eager_load<G: QueryGenerator>(
        _entities: &mut [Self],
        _relation: &str,
        _executor: &dyn QueryExecutor,
        _generator: &G,
    ) -> impl std::future::Future<Output = TikalResult<()>> + Send {
        async { Ok(()) }
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn table_definition() -> TableDefinition;

    fn generate_create_table_sql(driver: &str) -> String;

    fn find() -> QueryBuilder<Self> {
        QueryBuilder::new()
    }
}
