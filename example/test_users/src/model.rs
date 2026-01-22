use tikal::prelude::*;
use chrono::prelude::*;
#[derive(Entity, FromRow)]
#[table_name = "users"]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}