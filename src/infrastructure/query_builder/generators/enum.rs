use super::databases::{MySqlGenerator, PostgresGenerator, SqliteGenerator};
use super::SqlGenerator;
use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::value_objects::Value;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

#[derive(Clone)]
pub enum SqlGeneratorEnum {
    Postgres(PostgresGenerator),
    MySql(MySqlGenerator),
    Sqlite(SqliteGenerator),
}

impl SqlGeneratorEnum {}

impl SqlGenerator for SqlGeneratorEnum {
    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_select(builder),
            SqlGeneratorEnum::MySql(g) => g.generate_select(builder),
            SqlGeneratorEnum::Sqlite(g) => g.generate_select(builder),
        }
    }

    fn generate_insert<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_insert(entity),
            SqlGeneratorEnum::MySql(g) => g.generate_insert(entity),
            SqlGeneratorEnum::Sqlite(g) => g.generate_insert(entity),
        }
    }

    fn generate_update<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_update(entity),
            SqlGeneratorEnum::MySql(g) => g.generate_update(entity),
            SqlGeneratorEnum::Sqlite(g) => g.generate_update(entity),
        }
    }

    fn generate_delete<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_delete(entity),
            SqlGeneratorEnum::MySql(g) => g.generate_delete(entity),
            SqlGeneratorEnum::Sqlite(g) => g.generate_delete(entity),
        }
    }

    fn generate_create_table(&self, table: &TableDefinition) -> String {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_create_table(table),
            SqlGeneratorEnum::MySql(g) => g.generate_create_table(table),
            SqlGeneratorEnum::Sqlite(g) => g.generate_create_table(table),
        }
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_drop_table(table_name),
            SqlGeneratorEnum::MySql(g) => g.generate_drop_table(table_name),
            SqlGeneratorEnum::Sqlite(g) => g.generate_drop_table(table_name),
        }
    }

    fn generate_count<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_count(builder),
            SqlGeneratorEnum::MySql(g) => g.generate_count(builder),
            SqlGeneratorEnum::Sqlite(g) => g.generate_count(builder),
        }
    }

    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_sum(builder, field),
            SqlGeneratorEnum::MySql(g) => g.generate_sum(builder, field),
            SqlGeneratorEnum::Sqlite(g) => g.generate_sum(builder, field),
        }
    }

    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_avg(builder, field),
            SqlGeneratorEnum::MySql(g) => g.generate_avg(builder, field),
            SqlGeneratorEnum::Sqlite(g) => g.generate_avg(builder, field),
        }
    }

    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_min(builder, field),
            SqlGeneratorEnum::MySql(g) => g.generate_min(builder, field),
            SqlGeneratorEnum::Sqlite(g) => g.generate_min(builder, field),
        }
    }

    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.generate_max(builder, field),
            SqlGeneratorEnum::MySql(g) => g.generate_max(builder, field),
            SqlGeneratorEnum::Sqlite(g) => g.generate_max(builder, field),
        }
    }

    fn placeholder(&self, index: usize) -> String {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.placeholder(index),
            SqlGeneratorEnum::MySql(g) => g.placeholder(index),
            SqlGeneratorEnum::Sqlite(g) => g.placeholder(index),
        }
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.quote_identifier(identifier),
            SqlGeneratorEnum::MySql(g) => g.quote_identifier(identifier),
            SqlGeneratorEnum::Sqlite(g) => g.quote_identifier(identifier),
        }
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        match self {
            SqlGeneratorEnum::Postgres(g) => g.map_type(col_type),
            SqlGeneratorEnum::MySql(g) => g.map_type(col_type),
            SqlGeneratorEnum::Sqlite(g) => g.map_type(col_type),
        }
    }
}
