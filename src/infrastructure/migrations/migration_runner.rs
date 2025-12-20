pub struct MigrationRunner;
use crate::infrastructure::types::DbResult;
use crate::infrastructure::migrations::migration_status::MigrationStatus;


impl MigrationRunner {
    pub async fn run_pending(&self) -> DbResult<()> {
        Ok(())
    }

    pub async fn rollback(&self) -> DbResult<()> {
        Ok(())
    }
    pub async fn status(&self) -> DbResult<Vec<MigrationStatus>> {
        Ok(Vec::new())
    }
}