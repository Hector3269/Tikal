use crate::domain::query::builder::QueryBuilder;
use crate::domain::TikalResult;
use crate::infrastructure::query_builder::generators::SqlGeneratorEnum;
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

    fn eager_load(
        _entities: &mut [Self],
        _relation: &str,
        _executor: &dyn crate::infrastructure::drivers::traits::QueryExecutor,
        _generator: &SqlGeneratorEnum,
    ) -> impl std::future::Future<Output = TikalResult<()>> + Send {
        async { Ok(()) }
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn find() -> QueryBuilder<Self> {
        QueryBuilder::new()
    }
}
