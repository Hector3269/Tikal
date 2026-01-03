use super::SqlGenerator;
use crate::domain::model::Entity;
use crate::domain::query::builder::{Operator, OrderDirection, QueryBuilder};
use crate::domain::value_objects::Value;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

#[derive(Clone)]
pub struct MySqlGenerator;

impl SqlGenerator for MySqlGenerator {
    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT * FROM `{}`", E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            let mut where_clauses = Vec::new();
            for filter in &builder.filters {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let placeholders = vec!["?"; filter.values.len()].join(", ");
                        where_clauses.push(format!("`{}` IN ({})", filter.column, placeholders));
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                where_clauses.push(format!("`{}` {} ?", filter.column, op));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        if !builder.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_clauses: Vec<String> = builder
                .order_by
                .iter()
                .map(|o| {
                    let dir = match o.direction {
                        OrderDirection::Asc => "ASC",
                        OrderDirection::Desc => "DESC",
                    };
                    format!("`{}` {}", o.column, dir)
                })
                .collect();
            sql.push_str(&order_clauses.join(", "));
        }

        if let Some(limit) = builder.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = builder.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, params)
    }

    fn generate_count<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT COUNT(*) FROM `{}`", E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            let mut where_clauses = Vec::new();
            for filter in &builder.filters {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let placeholders = vec!["?"; filter.values.len()].join(", ");
                        where_clauses.push(format!("`{}` IN ({})", filter.column, placeholders));
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                where_clauses.push(format!("`{}` {} ?", filter.column, op));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        (sql, params)
    }

    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT SUM(`{}`) FROM `{}`", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            let mut where_clauses = Vec::new();
            for filter in &builder.filters {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let placeholders = vec!["?"; filter.values.len()].join(", ");
                        where_clauses.push(format!("`{}` IN ({})", filter.column, placeholders));
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                where_clauses.push(format!("`{}` {} ?", filter.column, op));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        (sql, params)
    }

    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT AVG(`{}`) FROM `{}`", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            let mut where_clauses = Vec::new();
            for filter in &builder.filters {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let placeholders = vec!["?"; filter.values.len()].join(", ");
                        where_clauses.push(format!("`{}` IN ({})", filter.column, placeholders));
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                where_clauses.push(format!("`{}` {} ?", filter.column, op));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        (sql, params)
    }

    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT MIN(`{}`) FROM `{}`", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            let mut where_clauses = Vec::new();
            for filter in &builder.filters {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let placeholders = vec!["?"; filter.values.len()].join(", ");
                        where_clauses.push(format!("`{}` IN ({})", filter.column, placeholders));
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                where_clauses.push(format!("`{}` {} ?", filter.column, op));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        (sql, params)
    }

    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT MAX(`{}`) FROM `{}`", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            let mut where_clauses = Vec::new();
            for filter in &builder.filters {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let placeholders = vec!["?"; filter.values.len()].join(", ");
                        where_clauses.push(format!("`{}` IN ({})", filter.column, placeholders));
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                where_clauses.push(format!("`{}` {} ?", filter.column, op));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        (sql, params)
    }

    fn generate_insert<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let values = entity.to_values();
        let mut columns = Vec::new();
        let mut placeholders = Vec::new();
        let mut params = Vec::new();

        for (column, value) in values {
            columns.push(format!("`{}`", column));
            placeholders.push("?".to_string());
            params.push(value);
        }

        let sql = format!(
            "INSERT INTO `{}` ({}) VALUES ({})",
            E::table_name(),
            columns.join(", "),
            placeholders.join(", ")
        );

        (sql, params)
    }

    fn generate_update<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let values = entity.to_values();
        let mut assignments = Vec::new();
        let mut params = Vec::new();
        let pk_name = E::primary_key();
        let mut pk_value = Value::Null;

        for (column, value) in values {
            if column == pk_name {
                pk_value = value;
                continue;
            }
            assignments.push(format!("`{}` = ?", column));
            params.push(value);
        }

        params.push(pk_value);
        let sql = format!(
            "UPDATE `{}` SET {} WHERE `{}` = ?",
            E::table_name(),
            assignments.join(", "),
            pk_name
        );

        (sql, params)
    }

    fn generate_delete<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let values = entity.to_values();
        let pk_name = E::primary_key();
        let pk_value = values.get(pk_name).cloned().unwrap_or(Value::Null);

        let sql = format!("DELETE FROM `{}` WHERE `{}` = ?", E::table_name(), pk_name);
        (sql, vec![pk_value])
    }

    fn generate_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS `{}` (", table.name);
        let mut column_defs = Vec::new();

        for col in &table.columns {
            let mut def = format!("`{}` {}", col.name, self.map_type(&col.column_type));

            if col.primary_key {
                def.push_str(" PRIMARY KEY");
                if matches!(col.column_type, ColumnType::Id) {
                    def.push_str(" AUTO_INCREMENT");
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
        sql.push_str(") ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci");
        sql
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS `{}`", table_name)
    }

    fn placeholder(&self, _index: usize) -> String {
        "?".to_string()
    }
}

impl MySqlGenerator {
    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        match col_type {
            ColumnType::Id => "BIGINT",
            ColumnType::Text => "VARCHAR(255)",
            ColumnType::LongText => "LONGTEXT",
            ColumnType::Int => "INT",
            ColumnType::BigInt => "BIGINT",
            ColumnType::Float => "DOUBLE",
            ColumnType::Bool => "BOOLEAN",
            ColumnType::DateTime => "DATETIME",
            ColumnType::NaiveDateTime => "DATETIME",
            ColumnType::Json => "JSON",
            ColumnType::Binary => "LONGBLOB",
        }
    }
}
