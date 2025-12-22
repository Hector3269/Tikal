use super::{SchemaBuilder, SqlGenerator};
use crate::domain::QueryExecutor;
use crate::infrastructure::types::DbResult;

pub struct ExecutableSchema<G: SqlGenerator> {
    builder: SchemaBuilder,
    generator: G,
}

impl<G: SqlGenerator> ExecutableSchema<G> {
    pub fn new(builder: SchemaBuilder, generator: G) -> Self {
        Self { builder, generator }
    }

    pub async fn execute<E>(&self, executor: &E) -> DbResult<()>
    where
        E: QueryExecutor,
    {
        for table in self.builder.tables() {
            let sql = self.generator.create_table(table);
            executor.execute_raw(&sql, &[]).await?;

            for index in &table.indexes {
                let sql = self.generator.create_index(table.name.as_str(), index);
                executor.execute_raw(&sql, &[]).await?;
            }
        }
        Ok(())
    }
}
