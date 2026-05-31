use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::value_objects::Value;
use crate::infrastructure::query_builder::ast::*;
use crate::infrastructure::query_builder::builders::QueryAstBuilder;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

pub trait SqlGenerator {
    fn placeholder(&self, index: usize) -> String;
    fn quote_identifier(&self, identifier: &str) -> String;
    fn map_type(&self, col_type: &ColumnType) -> &'static str;

    fn primary_key_suffix(&self) -> String {
        String::new()
    }

    fn table_options(&self) -> String {
        String::new()
    }

    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_select(builder);
        self.generate_select_ast(&ast)
    }

    fn generate_count<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_count(builder);
        self.generate_select_ast(&ast)
    }

    fn generate_aggregate<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        function: &str,
        field: &str,
    ) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_aggregate(builder, function, field);
        self.generate_select_ast(&ast)
    }

    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.generate_aggregate(builder, "SUM", field)
    }

    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.generate_aggregate(builder, "AVG", field)
    }

    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.generate_aggregate(builder, "MIN", field)
    }

    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        self.generate_aggregate(builder, "MAX", field)
    }

    fn generate_insert<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_insert(entity);
        self.generate_insert_ast(&ast)
    }

    fn generate_update<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_update(entity);
        self.generate_update_ast(&ast)
    }

    fn generate_delete<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_delete(entity);
        self.generate_delete_ast(&ast)
    }

    fn generate_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (",
            self.quote_identifier(&table.name)
        );

        let column_defs: Vec<String> = table
            .columns
            .iter()
            .map(|col| self.generate_column_definition(col))
            .collect();

        sql.push_str(&column_defs.join(", "));
        sql.push_str(")");
        sql.push_str(&self.table_options());
        sql
    }

    fn generate_column_definition(
        &self,
        col: &crate::infrastructure::schema::ColumnDefinition,
    ) -> String {
        let mut def = format!(
            "{} {}",
            self.quote_identifier(&col.name),
            self.map_type(&col.column_type)
        );

        if col.primary_key {
            def.push_str(" PRIMARY KEY");
            if col.column_type == ColumnType::Id {
                def.push_str(&self.primary_key_suffix());
            }
        } else {
            if !col.nullable {
                def.push_str(" NOT NULL");
            }
            if col.unique {
                def.push_str(" UNIQUE");
            }
        }

        if let Some(default) = &col.default_value {
            def.push_str(&format!(" DEFAULT {}", default));
        }

        def
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS {}", self.quote_identifier(table_name))
    }

    fn generate_select_ast(&self, query: &SelectQuery) -> (String, Vec<Value>) {
        super::common::CommonGenerator::generate_select(self, query)
    }

    fn generate_insert_ast(&self, query: &InsertQuery) -> (String, Vec<Value>) {
        super::common::CommonGenerator::generate_insert(self, query)
    }

    fn generate_update_ast(&self, query: &UpdateQuery) -> (String, Vec<Value>) {
        super::common::CommonGenerator::generate_update(self, query)
    }

    fn generate_delete_ast(&self, query: &DeleteQuery) -> (String, Vec<Value>) {
        super::common::CommonGenerator::generate_delete(self, query)
    }
}
