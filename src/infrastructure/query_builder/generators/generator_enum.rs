use super::base::BaseGenerator;
use super::sql_generator::SqlGenerator;
use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::query_generator::QueryGenerator;
use crate::domain::value_objects::Value;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

#[derive(Clone)]
pub enum SqlGeneratorEnum {
    Postgres(BaseGenerator),
    MySql(BaseGenerator),
    Sqlite(BaseGenerator),
}

impl SqlGeneratorEnum {
    pub fn postgres() -> Self {
        Self::Postgres(BaseGenerator::postgres())
    }

    pub fn mysql() -> Self {
        Self::MySql(BaseGenerator::mysql())
    }

    pub fn sqlite() -> Self {
        Self::Sqlite(BaseGenerator::sqlite())
    }

    pub fn from_driver(driver: &str) -> Option<Self> {
        match driver {
            "postgres" | "postgresql" => Some(Self::postgres()),
            "mysql" => Some(Self::mysql()),
            "sqlite" => Some(Self::sqlite()),
            _ => None,
        }
    }

    fn get_generator(&self) -> &BaseGenerator {
        match self {
            Self::Postgres(g) => g,
            Self::MySql(g) => g,
            Self::Sqlite(g) => g,
        }
    }
}

impl QueryGenerator for SqlGeneratorEnum {
    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        self.get_generator().generate_select(builder)
    }

    fn generate_insert<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        self.get_generator().generate_insert(entity)
    }

    fn generate_update<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        self.get_generator().generate_update(entity)
    }

    fn generate_delete<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        self.get_generator().generate_delete(entity)
    }

    fn generate_count<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        self.get_generator().generate_count(builder)
    }

    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.get_generator().generate_sum(builder, field)
    }

    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.get_generator().generate_avg(builder, field)
    }

    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.get_generator().generate_min(builder, field)
    }

    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.get_generator().generate_max(builder, field)
    }
}

impl SqlGenerator for SqlGeneratorEnum {
    fn placeholder(&self, index: usize) -> String {
        self.get_generator().placeholder(index)
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        self.get_generator().quote_identifier(identifier)
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        self.get_generator().map_type(col_type)
    }

    fn primary_key_suffix(&self) -> String {
        self.get_generator().primary_key_suffix()
    }

    fn table_options(&self) -> String {
        self.get_generator().table_options()
    }

    fn generate_create_table(&self, table: &TableDefinition) -> String {
        self.get_generator().generate_create_table(table)
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        self.get_generator().generate_drop_table(table_name)
    }
}

pub type MySqlGenerator = BaseGenerator;
pub type PostgresGenerator = BaseGenerator;
pub type SqliteGenerator = BaseGenerator;

impl BaseGenerator {
    pub fn for_driver(driver: &str) -> Option<Self> {
        match driver {
            "postgres" | "postgresql" => Some(Self::postgres()),
            "mysql" => Some(Self::mysql()),
            "sqlite" => Some(Self::sqlite()),
            _ => None,
        }
    }
}
