use crate::domain::TikalResult;
use crate::infrastructure::drivers::traits::QueryExecutor;
use crate::infrastructure::migrations::manager::MigrationManager;
use crate::infrastructure::migrations::Migration;
use crate::infrastructure::query_builder::generators::SqlGenerator;

pub struct MigrationRunner<'a, G: SqlGenerator> {
    executor: &'a dyn QueryExecutor,
    manager: MigrationManager<'a, G>,
    migrations: Vec<Box<dyn Migration + 'a>>,
}

impl<'a, G: SqlGenerator> MigrationRunner<'a, G> {
    pub fn new(
        executor: &'a dyn QueryExecutor,
        generator: &'a G,
        migrations: Vec<Box<dyn Migration + 'a>>,
    ) -> Self {
        let manager = MigrationManager::new(executor, generator);
        Self {
            executor,
            manager,
            migrations,
        }
    }

    pub async fn run_pending_migrations(&self) -> TikalResult<()> {
        self.manager.create_migrations_table().await?;

        let applied = self.manager.get_applied_migrations().await?;

        let mut sorted_migrations: Vec<_> = self.migrations.iter().collect();
        sorted_migrations.sort_by_key(|m| m.version());

        for migration in sorted_migrations {
            let name = migration.name();
            let version = migration.version();

            if let Some(applied_version) = applied.get(name) {
                if *applied_version >= version {
                    continue; // Already applied
                }
            }

            migration.up(self.executor).await?;

            self.manager.mark_migration_applied(name, version).await?;
        }

        Ok(())
    }
}
