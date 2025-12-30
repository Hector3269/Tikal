use crate::kernel::types::schema::MigrationName;
pub struct MigrationStatus {
    pub name: MigrationName,
    pub executed: bool,
    pub executed_at: Option<chrono::DateTime<chrono::Utc>>,
}