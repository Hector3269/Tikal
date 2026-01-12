use crate::domain::model::Entity;
use crate::domain::repositories::{executor::QueryExecutor, Repository};
use crate::domain::{query::builder::QueryBuilder, TikalResult};
use crate::infrastructure::database::DatabasePool;
use crate::infrastructure::query_builder::generators::SqlGeneratorEnum;
use crate::infrastructure::repositories::SqlRepository;

#[cfg(feature = "mysql")]
use crate::infrastructure::drivers::mysql::MySqlExecutor;

#[cfg(feature = "postgres")]
use crate::infrastructure::drivers::postgres::PostgresExecutor;

#[cfg(feature = "sqlite")]
use crate::infrastructure::drivers::sqlite::SqliteExecutor;

pub struct TikalApp {
    pub pool: DatabasePool,
}

impl TikalApp {
    pub fn new(pool: DatabasePool) -> Self {
        Self { pool }
    }

    pub fn repository<E: Entity + 'static>(&self) -> Box<dyn Repository<E>> {
        match &self.pool {
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                let repo = SqlRepository::new(SqlGeneratorEnum::mysql(), executor);
                Box::new(repo)
            }
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                let repo = SqlRepository::new(SqlGeneratorEnum::postgres(), executor);
                Box::new(repo)
            }
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                let repo = SqlRepository::new(SqlGeneratorEnum::sqlite(), executor);
                Box::new(repo)
            }
        }
    }

    pub async fn save<E: Entity + 'static>(&self, entity: &E) -> TikalResult<u64> {
        let repo = self.repository::<E>();
        repo.save(entity).await
    }

    pub async fn find_all<E: Entity + 'static>(&self) -> TikalResult<Vec<E>> {
        let repo = self.repository::<E>();
        repo.find_all().await
    }

    pub async fn find_by_id<E: Entity + 'static>(
        &self,
        id: &crate::domain::value_objects::Value,
    ) -> TikalResult<Option<E>> {
        let repo = self.repository::<E>();
        repo.find_by_id(id).await
    }

    pub async fn find_with_query<E: Entity + 'static>(
        &self,
        query: QueryBuilder<E>,
    ) -> TikalResult<Vec<E>> {
        let repo = self.repository::<E>();
        repo.find_with_query(query).await
    }

    pub async fn update<E: Entity + 'static>(&self, entity: &E) -> TikalResult<u64> {
        let repo = self.repository::<E>();
        repo.update(entity).await
    }

    pub async fn delete<E: Entity + 'static>(&self, entity: &E) -> TikalResult<u64> {
        let repo = self.repository::<E>();
        repo.delete(entity).await
    }

    pub async fn count<E: Entity + 'static>(&self, query: QueryBuilder<E>) -> TikalResult<i64> {
        let repo = self.repository::<E>();
        repo.count(query).await
    }

    pub async fn execute_raw(
        &self,
        sql: &str,
        params: Vec<crate::domain::value_objects::Value>,
    ) -> TikalResult<u64> {
        match &self.pool {
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                executor.execute(sql, params).await
            }
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                executor.execute(sql, params).await
            }
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                executor.execute(sql, params).await
            }
        }
    }

    pub async fn query_raw(
        &self,
        sql: &str,
        params: Vec<crate::domain::value_objects::Value>,
    ) -> TikalResult<Vec<std::collections::HashMap<String, crate::domain::value_objects::Value>>>
    {
        match &self.pool {
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                executor.fetch_all(sql, params).await
            }
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                executor.fetch_all(sql, params).await
            }
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                executor.fetch_all(sql, params).await
            }
        }
    }

    pub fn driver_info(&self) -> crate::domain::repositories::types::DriverInfo {
        match &self.pool {
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                executor.driver_info()
            }
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                executor.driver_info()
            }
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                executor.driver_info()
            }
        }
    }

    pub async fn begin_transaction(
        &self,
    ) -> TikalResult<Box<dyn crate::domain::repositories::executor::Transaction>> {
        match &self.pool {
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                executor.begin_transaction().await
            }
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                executor.begin_transaction().await
            }
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                executor.begin_transaction().await
            }
        }
    }

    pub async fn ping(&self) -> TikalResult<bool> {
        match &self.pool {
            DatabasePool::MySql(pool) => {
                let executor = MySqlExecutor::new((**pool).clone());
                executor.ping().await
            }
            DatabasePool::Postgres(pool) => {
                let executor = PostgresExecutor::new((**pool).clone());
                executor.ping().await
            }
            DatabasePool::Sqlite(pool) => {
                let executor = SqliteExecutor::new((**pool).clone());
                executor.ping().await
            }
        }
    }

    pub async fn save_many<E: Entity + 'static>(&self, entities: &[E]) -> TikalResult<u64> {
        let repo = self.repository::<E>();
        repo.save_many(entities).await
    }

    pub async fn update_many<E: Entity + 'static>(&self, entities: &[E]) -> TikalResult<u64> {
        let repo = self.repository::<E>();
        repo.update_many(entities).await
    }

    pub async fn delete_many<E: Entity + 'static>(&self, entities: &[E]) -> TikalResult<u64> {
        let repo = self.repository::<E>();
        repo.delete_many(entities).await
    }
}
