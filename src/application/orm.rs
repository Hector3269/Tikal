use crate::domain::model::Entity;
use crate::domain::{
    query::builder::{Operator, QueryBuilder},
    value_objects::Value,
    TikalResult,
};
use crate::infrastructure::database::DatabasePool;
use crate::infrastructure::drivers::traits::QueryExecutor;
use crate::infrastructure::query_builder::generators::{SqlGenerator, SqlGeneratorEnum};
use crate::infrastructure::repositories::SqlRepository;

#[cfg(feature = "mysql")]
use crate::infrastructure::drivers::mysql::MySqlExecutor;
#[cfg(feature = "mysql")]
use crate::infrastructure::query_builder::generators::MySqlGenerator;

#[cfg(feature = "postgres")]
use crate::infrastructure::drivers::postgres::PostgresExecutor;
#[cfg(feature = "postgres")]
use crate::infrastructure::query_builder::generators::PostgresGenerator;

#[cfg(feature = "sqlite")]
use crate::infrastructure::drivers::sqlite::SqliteExecutor;
#[cfg(feature = "sqlite")]
use crate::infrastructure::query_builder::generators::SqliteGenerator;

pub struct TikalApp {
    pub pool: DatabasePool,
}

impl TikalApp {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    #[cfg(feature = "mysql")]
    pub fn mysql_repository<E: Entity>(&self) -> SqlRepository<E, MySqlExecutor> {
        let executor = match &self.pool {
            DatabasePool::MySql(pool) => MySqlExecutor::new((**pool).clone()),
            _ => panic!("Invalid pool type for MySQL repository"),
        };
        SqlRepository::new(SqlGeneratorEnum::MySql(MySqlGenerator), executor)
    }

    #[cfg(feature = "postgres")]
    pub fn postgres_repository<E: Entity>(&self) -> SqlRepository<E, PostgresExecutor> {
        let executor = match &self.pool {
            DatabasePool::Postgres(pool) => PostgresExecutor::new((**pool).clone()),
            _ => panic!("Invalid pool type for PostgreSQL repository"),
        };
        SqlRepository::new(SqlGeneratorEnum::Postgres(PostgresGenerator), executor)
    }

    #[cfg(feature = "sqlite")]
    pub fn sqlite_repository<E: Entity>(&self) -> SqlRepository<E, SqliteExecutor> {
        let executor = match &self.pool {
            DatabasePool::Sqlite(pool) => SqliteExecutor::new((**pool).clone()),
            _ => panic!("Invalid pool type for SQLite repository"),
        };
        SqlRepository::new(SqlGeneratorEnum::Sqlite(SqliteGenerator), executor)
    }

    pub async fn save<E: Entity>(&self, entity: &E) -> TikalResult<u64> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Sqlite(SqliteGenerator);
                let (sql, params) = generator.generate_insert(entity);
                executor.execute(&sql, params).await
            }
            #[cfg(feature = "mysql")]
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::MySql(MySqlGenerator);
                let (sql, params) = generator.generate_insert(entity);
                executor.execute(&sql, params).await
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Postgres(PostgresGenerator);
                let (sql, params) = generator.generate_insert(entity);
                executor.execute(&sql, params).await
            }
        }
    }

    pub async fn find_all<E: Entity>(&self) -> TikalResult<Vec<E>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Sqlite(SqliteGenerator);
                let query = E::find();
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                let mut entities = Vec::new();
                for row in rows {
                    entities.push(E::from_row(row)?);
                }
                Ok(entities)
            }
            #[cfg(feature = "mysql")]
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::MySql(MySqlGenerator);
                let query = E::find();
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                let mut entities = Vec::new();
                for row in rows {
                    entities.push(E::from_row(row)?);
                }
                Ok(entities)
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Postgres(PostgresGenerator);
                let query = E::find();
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                let mut entities = Vec::new();
                for row in rows {
                    entities.push(E::from_row(row)?);
                }
                Ok(entities)
            }
        }
    }

    pub async fn find_by_id<E: Entity>(&self, id: &Value) -> TikalResult<Option<E>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Sqlite(SqliteGenerator);
                let query = E::find().where_clause(E::primary_key(), Operator::Eq, id.clone());
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                if rows.is_empty() {
                    Ok(None)
                } else {
                    let entity = E::from_row(rows.into_iter().next().unwrap())?;
                    Ok(Some(entity))
                }
            }
            #[cfg(feature = "mysql")]
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::MySql(MySqlGenerator);
                let query = E::find().where_clause(E::primary_key(), Operator::Eq, id.clone());
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                if rows.is_empty() {
                    Ok(None)
                } else {
                    let entity = E::from_row(rows.into_iter().next().unwrap())?;
                    Ok(Some(entity))
                }
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Postgres(PostgresGenerator);
                let query = E::find().where_clause(E::primary_key(), Operator::Eq, id.clone());
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                if rows.is_empty() {
                    Ok(None)
                } else {
                    let entity = E::from_row(rows.into_iter().next().unwrap())?;
                    Ok(Some(entity))
                }
            }
        }
    }

    pub async fn find_with_query<E: Entity>(&self, query: QueryBuilder<E>) -> TikalResult<Vec<E>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Sqlite(SqliteGenerator);
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                let mut entities = Vec::new();
                for row in rows {
                    entities.push(E::from_row(row)?);
                }
                Ok(entities)
            }
            #[cfg(feature = "mysql")]
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::MySql(MySqlGenerator);
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                let mut entities = Vec::new();
                for row in rows {
                    entities.push(E::from_row(row)?);
                }
                Ok(entities)
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Postgres(PostgresGenerator);
                let (sql, params) = generator.generate_select(&query);
                let rows = executor.fetch_all(&sql, params).await?;
                let mut entities = Vec::new();
                for row in rows {
                    entities.push(E::from_row(row)?);
                }
                Ok(entities)
            }
        }
    }

    pub async fn update<E: Entity>(&self, entity: &E) -> TikalResult<u64> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Sqlite(SqliteGenerator);
                let (sql, params) = generator.generate_update(entity);
                executor.execute(&sql, params).await
            }
            #[cfg(feature = "mysql")]
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::MySql(MySqlGenerator);
                let (sql, params) = generator.generate_update(entity);
                executor.execute(&sql, params).await
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Postgres(PostgresGenerator);
                let (sql, params) = generator.generate_update(entity);
                executor.execute(&sql, params).await
            }
        }
    }

    pub async fn delete<E: Entity>(&self, entity: &E) -> TikalResult<u64> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Sqlite(SqliteGenerator);
                let (sql, params) = generator.generate_delete(entity);
                executor.execute(&sql, params).await
            }
            #[cfg(feature = "mysql")]
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::MySql(MySqlGenerator);
                let (sql, params) = generator.generate_delete(entity);
                executor.execute(&sql, params).await
            }
            #[cfg(feature = "postgres")]
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let generator = SqlGeneratorEnum::Postgres(PostgresGenerator);
                let (sql, params) = generator.generate_delete(entity);
                executor.execute(&sql, params).await
            }
        }
    }
}
