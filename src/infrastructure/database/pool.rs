use std::sync::Arc;

#[cfg(feature = "mysql")]
use sqlx::MySqlPool;
#[cfg(feature = "postgres")]
use sqlx::PgPool;
#[cfg(feature = "sqlite")]
use sqlx::SqlitePool;

#[derive(Clone)]
pub enum DatabasePool {
    #[cfg(feature = "mysql")]
    MySql(Arc<MySqlPool>),
    #[cfg(feature = "postgres")]
    Postgres(Arc<PgPool>),
    #[cfg(feature = "sqlite")]
    Sqlite(Arc<SqlitePool>),
}
