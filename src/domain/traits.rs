use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;
use crate::kernel::types::db::DbRow;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait QueryExecutor {
    async fn execute_raw(&self, sql: &str, params: &[Value]) -> Result<(), KernelError>;
    async fn query_raw(&self, sql: &str, params: &[Value]) -> Result<Vec<DbRow>, KernelError>;
    async fn transaction(
        &self,
        f: Box<
            dyn FnOnce() -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), KernelError>> + Send>,
                > + Send,
        >,
    ) -> Result<(), KernelError>;
    async fn savepoint(
        &self,
        name: &str,
        f: Box<
            dyn FnOnce() -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = Result<(), KernelError>> + Send>,
                > + Send,
        >,
    ) -> Result<(), KernelError>;
    async fn query_stream(
        &self,
        sql: &str,
        params: &[Value],
        callback: Box<dyn Fn(DbRow) -> Result<(), KernelError> + Send + Sync>,
    ) -> Result<(), KernelError>;
}

pub trait FromRow {
    fn from_row(row: HashMap<String, Value>) -> Result<Self, KernelError>
    where
        Self: Sized;
}

pub trait Entity: Send + Sync {
    fn entity_name() -> &'static str;
}

pub trait Identifiable {
    fn id(&self) -> &str;
}

/// ActiveRecord pattern for models that handle their own persistence
#[async_trait]
pub trait ActiveRecord: Entity + Identifiable + Sized {
    /// Returns the table name for this model
    fn table_name() -> String;

    /// Returns the primary key column name
    fn primary_key() -> String;

    /// Creates an instance from a database row
    fn from_row(row: DbRow) -> Result<Self, KernelError>;

    /// Returns model attributes as a map
    fn attributes(&self) -> HashMap<String, Value>;

    /// Find by ID
    async fn find<E: QueryExecutor + Send + Sync>(
        executor: &E,
        id: &str,
    ) -> Result<Option<Self>, crate::kernel::error::KernelError>;

    /// Save the entity
    async fn save<E: QueryExecutor + Send + Sync>(
        &mut self,
        executor: &E,
    ) -> Result<(), crate::kernel::error::KernelError>;

    /// Delete the entity
    async fn delete<E: QueryExecutor + Send + Sync>(
        &self,
        executor: &E,
    ) -> Result<(), crate::kernel::error::KernelError>;
}

/// Trait for automatic timestamps
pub trait Timestamps {
    fn uses_timestamps() -> bool {
        true
    }
    fn fresh_timestamp() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
    fn get_created_at(&self) -> &Option<chrono::DateTime<chrono::Utc>>;
    fn get_updated_at(&self) -> &Option<chrono::DateTime<chrono::Utc>>;
    fn set_created_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>);
    fn set_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>);
}

/// Trait for soft deletes
pub trait SoftDeletes {
    fn uses_soft_deletes() -> bool {
        true
    }
    fn get_deleted_at(&self) -> &Option<chrono::DateTime<chrono::Utc>>;
    fn set_deleted_at(&mut self, timestamp: Option<chrono::DateTime<chrono::Utc>>);
}

/// Trait for global query scopes
pub trait GlobalScope<T: Model> {
    fn apply(
        &self,
        query: &mut crate::domain::queries::QueryBuilder<T>,
    ) -> impl std::future::Future<Output = ()> + Send;
}

/// Trait for local query scopes
pub trait Scopes {
    fn scope_active(self) -> Self;
    fn scope_published(self) -> Self;
    // Add more scopes as needed
}

/// Model trait for Eloquent-like querying
#[async_trait(?Send)]
pub trait Model:
    ActiveRecord + From<DbRow> + Timestamps + SoftDeletes + Send + Sync + 'static
{
    /// Create a query builder for this model
    fn query() -> crate::domain::queries::QueryBuilder<Self> {
        crate::domain::queries::QueryBuilder::new()
    }

    /// Find by ID (convenience method)
    async fn find_by_id<E: QueryExecutor + Send + Sync>(
        executor: &E,
        id: &str,
    ) -> Result<Option<Self>, crate::kernel::error::KernelError> {
        Self::find(executor, id).await
    }

    /// Get all records
    async fn all<E: QueryExecutor + Send + Sync>(
        executor: &E,
    ) -> Result<Vec<Self>, crate::kernel::error::KernelError> {
        Self::query().get(executor).await
    }

    /// Get first record matching query
    async fn first<E: QueryExecutor + Send + Sync>(
        query: crate::domain::queries::QueryBuilder<Self>,
        executor: &E,
    ) -> Result<Option<Self>, crate::kernel::error::KernelError> {
        query.first(executor).await
    }

    /// Get records matching query
    async fn get<E: QueryExecutor + Send + Sync>(
        query: crate::domain::queries::QueryBuilder<Self>,
        executor: &E,
    ) -> Result<Vec<Self>, crate::kernel::error::KernelError> {
        query.get(executor).await
    }
}
