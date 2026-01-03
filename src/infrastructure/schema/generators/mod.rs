pub mod mysql;
pub mod postgres;
pub mod sqlite;

pub use mysql::MySqlDdlGenerator;
pub use postgres::PostgresDdlGenerator;
pub use sqlite::SqliteDdlGenerator;

use crate::infrastructure::schema::TableDefinition;

pub trait DdlGenerator {
    fn generate_create_table(&self, table: &TableDefinition) -> String;
    fn generate_drop_table(&self, table_name: &str) -> String;
    fn generate_create_index(
        &self,
        table_name: &str,
        index_name: &str,
        columns: &[String],
        unique: bool,
    ) -> String;
    fn generate_drop_index(&self, index_name: &str) -> String;
}
