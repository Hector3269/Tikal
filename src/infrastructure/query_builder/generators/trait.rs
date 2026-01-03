use super::super::ast::*;
use super::super::builders::QueryAstBuilder;
use super::common::CommonGenerator;
use crate::domain::model::Entity;
use crate::domain::query::builder::QueryBuilder;
use crate::domain::value_objects::Value;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

pub trait SqlGenerator {
    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_select(builder);
        self.generate_select_ast(&ast)
    }

    fn generate_count<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_count(builder);
        self.generate_select_ast(&ast)
    }

    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_aggregate(builder, "SUM", field);
        self.generate_select_ast(&ast)
    }

    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_aggregate(builder, "AVG", field);
        self.generate_select_ast(&ast)
    }

    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_aggregate(builder, "MIN", field);
        self.generate_select_ast(&ast)
    }

    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let ast = QueryAstBuilder::build_aggregate(builder, "MAX", field);
        self.generate_select_ast(&ast)
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
        let mut column_defs = Vec::new();

        for col in &table.columns {
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

            column_defs.push(def);
        }

        sql.push_str(&column_defs.join(", "));
        sql.push_str(")");
        sql.push_str(&self.table_options());
        sql
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS {}", self.quote_identifier(table_name))
    }

    fn placeholder(&self, index: usize) -> String;

    fn quote_identifier(&self, identifier: &str) -> String {
        identifier.to_string()
    }

    fn map_type(&self, col_type: &ColumnType) -> &'static str;

    fn primary_key_suffix(&self) -> String {
        String::new()
    }

    fn table_options(&self) -> String {
        String::new()
    }

    fn generate_select_ast(&self, query: &SelectQuery) -> (String, Vec<Value>) {
        CommonGenerator::generate_select(self, query)
    }

    fn generate_insert_ast(&self, query: &InsertQuery) -> (String, Vec<Value>) {
        CommonGenerator::generate_insert(self, query)
    }

    fn generate_update_ast(&self, query: &UpdateQuery) -> (String, Vec<Value>) {
        CommonGenerator::generate_update(self, query)
    }

    fn generate_delete_ast(&self, query: &DeleteQuery) -> (String, Vec<Value>) {
        CommonGenerator::generate_delete(self, query)
    }
}
