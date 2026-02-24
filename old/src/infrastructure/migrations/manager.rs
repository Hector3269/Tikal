use crate::domain::TikalResult;
use crate::infrastructure::drivers::traits::QueryExecutor;
use crate::infrastructure::query_builder::generators::SqlGenerator;
use crate::infrastructure::schema::builders::TableBuilder;
use crate::infrastructure::schema::ColumnType;

pub struct MigrationManager<'a, G: SqlGenerator> {
    executor: &'a dyn QueryExecutor,
    generator: &'a G,
}

impl<'a, G: SqlGenerator> MigrationManager<'a, G> {
    pub fn new(executor: &'a dyn QueryExecutor, generator: &'a G) -> Self {
        Self {
            executor,
            generator,
        }
    }

    pub async fn create_migrations_table(&self) -> TikalResult<()> {
        let table = TableBuilder::new("__migrations")
            .id()
            .column("name", ColumnType::Text)
            .finish()
            .column("version", ColumnType::BigInt)
            .finish()
            .build();

        let sql = self.generator.generate_create_table(&table);
        self.executor.execute(&sql, vec![]).await?;
        Ok(())
    }

    pub async fn get_applied_migrations(
        &self,
    ) -> TikalResult<std::collections::HashMap<String, u64>> {
        let sql = "SELECT name, version FROM __migrations ORDER BY version ASC";
        let rows = self.executor.fetch_all(sql, vec![]).await?;
        let mut applied = std::collections::HashMap::new();
        for row in rows {
            let name_value = row.get("name").ok_or_else(|| {
                crate::domain::TikalError::mapping("migration", "missing name column")
            })?;
            let name = match name_value {
                crate::domain::value_objects::Value::Text(s) => s.clone(),
                _ => {
                    return Err(crate::domain::TikalError::mapping(
                        "migration",
                        "name is not text",
                    ))
                }
            };
            let version_value = row.get("version").ok_or_else(|| {
                crate::domain::TikalError::mapping("migration", "missing version column")
            })?;
            let version = match version_value {
                crate::domain::value_objects::Value::Int(i) => *i as u64,
                _ => {
                    return Err(crate::domain::TikalError::mapping(
                        "migration",
                        "version is not int",
                    ))
                }
            };
            applied.insert(name, version);
        }
        Ok(applied)
    }

    pub async fn mark_migration_applied(&self, name: &str, version: u64) -> TikalResult<()> {
        let sql = format!(
            "INSERT INTO __migrations (name, version) VALUES ({}, {})",
            self.generator.placeholder(1),
            self.generator.placeholder(2)
        );
        let params = vec![
            crate::domain::value_objects::Value::Text(name.to_string()),
            crate::domain::value_objects::Value::Int(version as i64),
        ];
        self.executor.execute(&sql, params).await?;
        Ok(())
    }

    pub async fn is_migration_applied(&self, name: &str, version: u64) -> TikalResult<bool> {
        let sql = format!(
            "SELECT COUNT(*) as count FROM __migrations WHERE name = {} AND version = {}",
            self.generator.placeholder(1),
            self.generator.placeholder(2)
        );
        let params = vec![
            crate::domain::value_objects::Value::Text(name.to_string()),
            crate::domain::value_objects::Value::Int(version as i64),
        ];
        let rows = self.executor.fetch_all(&sql, params).await?;
        if let Some(row) = rows.into_iter().next() {
            let count_value = row.get("count").ok_or_else(|| {
                crate::domain::TikalError::mapping("migration", "missing count column")
            })?;
            let count = match count_value {
                crate::domain::value_objects::Value::Int(i) => *i,
                _ => {
                    return Err(crate::domain::TikalError::mapping(
                        "migration",
                        "count is not int",
                    ))
                }
            };
            Ok(count > 0)
        } else {
            Ok(false)
        }
    }
}
