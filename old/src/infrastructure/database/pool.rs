use sqlx::{MySqlPool, PgPool, SqlitePool};
use std::sync::Arc;

#[derive(Clone)]
pub enum DatabasePool {
    #[cfg(feature = "mysql")]
    MySql(Arc<MySqlPool>),
    #[cfg(feature = "postgres")]
    Postgres(Arc<PgPool>),
    #[cfg(feature = "sqlite")]
    Sqlite(Arc<SqlitePool>),
}
