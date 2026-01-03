use super::SqlGenerator;
use crate::domain::model::Entity;
use crate::domain::query::builder::{Operator, OrderDirection, QueryBuilder};
use crate::domain::value_objects::Value;
use crate::infrastructure::schema::{ColumnType, TableDefinition};

#[derive(Clone)]
pub struct PostgresGenerator;

impl SqlGenerator for PostgresGenerator {
    fn generate_select<E: Entity>(&self, builder: &QueryBuilder<E>) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT * FROM {}", E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut p_index = 0;
            let mut where_clauses = Vec::new();

            for filter in builder.filters.iter() {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let mut in_placeholders = Vec::new();
                        for _ in &filter.values {
                            p_index += 1;
                            in_placeholders.push(format!("${}", p_index));
                        }
                        let sql_part =
                            format!("{} IN ({})", filter.column, in_placeholders.join(", "));
                        where_clauses.push(sql_part);
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                p_index += 1;
                where_clauses.push(format!("{} {} ${}", filter.column, op, p_index));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&where_clauses.join(" AND "));
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
                    format!("{} {}", o.column, dir)
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
        let mut sql = format!("SELECT COUNT(*) FROM {}", E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut p_index = 0;
            let mut where_clauses = Vec::new();

            for filter in builder.filters.iter() {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let mut in_placeholders = Vec::new();
                        for _ in &filter.values {
                            p_index += 1;
                            in_placeholders.push(format!("${}", p_index));
                        }
                        let sql_part =
                            format!("{} IN ({})", filter.column, in_placeholders.join(", "));
                        where_clauses.push(sql_part);
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                p_index += 1;
                where_clauses.push(format!("{} {} ${}", filter.column, op, p_index));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    fn generate_sum<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT SUM({}) FROM {}", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut p_index = 0;
            let mut where_clauses = Vec::new();

            for filter in builder.filters.iter() {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let mut in_placeholders = Vec::new();
                        for _ in &filter.values {
                            p_index += 1;
                            in_placeholders.push(format!("${}", p_index));
                        }
                        let sql_part =
                            format!("{} IN ({})", filter.column, in_placeholders.join(", "));
                        where_clauses.push(sql_part);
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                p_index += 1;
                where_clauses.push(format!("{} {} ${}", filter.column, op, p_index));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    fn generate_avg<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT AVG({}) FROM {}", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut p_index = 0;
            let mut where_clauses = Vec::new();

            for filter in builder.filters.iter() {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let mut in_placeholders = Vec::new();
                        for _ in &filter.values {
                            p_index += 1;
                            in_placeholders.push(format!("${}", p_index));
                        }
                        let sql_part =
                            format!("{} IN ({})", filter.column, in_placeholders.join(", "));
                        where_clauses.push(sql_part);
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                p_index += 1;
                where_clauses.push(format!("{} {} ${}", filter.column, op, p_index));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    fn generate_min<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT MIN({}) FROM {}", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut p_index = 0;
            let mut where_clauses = Vec::new();

            for filter in builder.filters.iter() {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let mut in_placeholders = Vec::new();
                        for _ in &filter.values {
                            p_index += 1;
                            in_placeholders.push(format!("${}", p_index));
                        }
                        let sql_part =
                            format!("{} IN ({})", filter.column, in_placeholders.join(", "));
                        where_clauses.push(sql_part);
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                p_index += 1;
                where_clauses.push(format!("{} {} ${}", filter.column, op, p_index));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    fn generate_max<E: Entity>(
        &self,
        builder: &QueryBuilder<E>,
        field: &str,
    ) -> (String, Vec<Value>) {
        let mut sql = format!("SELECT MAX({}) FROM {}", field, E::table_name());
        let mut params = Vec::new();

        if !builder.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut p_index = 0;
            let mut where_clauses = Vec::new();

            for filter in builder.filters.iter() {
                let op = match filter.operator {
                    Operator::Eq => "=",
                    Operator::Ne => "!=",
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Gte => ">=",
                    Operator::Lte => "<=",
                    Operator::Like => "LIKE",
                    Operator::In => {
                        let mut in_placeholders = Vec::new();
                        for _ in &filter.values {
                            p_index += 1;
                            in_placeholders.push(format!("${}", p_index));
                        }
                        let sql_part =
                            format!("{} IN ({})", filter.column, in_placeholders.join(", "));
                        where_clauses.push(sql_part);
                        params.extend(filter.values.clone());
                        continue;
                    }
                };

                p_index += 1;
                where_clauses.push(format!("{} {} ${}", filter.column, op, p_index));
                params.push(filter.values[0].clone());
            }
            sql.push_str(&where_clauses.join(" AND "));
        }

        (sql, params)
    }

    fn generate_insert<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let values = entity.to_values();
        let mut columns = Vec::new();
        let mut placeholders = Vec::new();
        let mut params = Vec::new();

        for (i, (column, value)) in values.into_iter().enumerate() {
            columns.push(column);
            placeholders.push(format!("${}", i + 1));
            params.push(value);
        }

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
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

        let mut i = 0;
        for (column, value) in values {
            if column == pk_name {
                pk_value = value;
                continue;
            }
            assignments.push(format!("{} = ${}", column, i + 1));
            params.push(value);
            i += 1;
        }

        params.push(pk_value);
        let sql = format!(
            "UPDATE {} SET {} WHERE {} = ${}",
            E::table_name(),
            assignments.join(", "),
            pk_name,
            i + 1
        );

        (sql, params)
    }

    fn generate_delete<E: Entity>(&self, entity: &E) -> (String, Vec<Value>) {
        let values = entity.to_values();
        let pk_name = E::primary_key();
        let pk_value = values.get(pk_name).cloned().unwrap_or(Value::Null);

        let sql = format!("DELETE FROM {} WHERE {} = $1", E::table_name(), pk_name);
        (sql, vec![pk_value])
    }

    fn generate_create_table(&self, table: &TableDefinition) -> String {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (", table.name);
        let mut column_defs = Vec::new();

        for col in &table.columns {
            let mut def = format!("{} ", col.name);

            if col.primary_key && matches!(col.column_type, ColumnType::Id) {
                def.push_str("BIGSERIAL PRIMARY KEY");
            } else {
                def.push_str(self.map_type(&col.column_type));

                if col.primary_key {
                    def.push_str(" PRIMARY KEY");
                } else {
                    if !col.nullable {
                        def.push_str(" NOT NULL");
                    }
                    if col.unique {
                        def.push_str(" UNIQUE");
                    }
                }
            }

            column_defs.push(def);
        }

        sql.push_str(&column_defs.join(", "));
        sql.push_str(")");
        sql
    }

    fn generate_drop_table(&self, table_name: &str) -> String {
        format!("DROP TABLE IF EXISTS {} CASCADE", table_name)
    }

    fn placeholder(&self, index: usize) -> String {
        format!("${}", index)
    }
}

impl PostgresGenerator {
    fn map_type(&self, col_type: &ColumnType) -> &'static str {
        match col_type {
            ColumnType::Id => "BIGINT",
            ColumnType::Text => "VARCHAR(255)",
            ColumnType::LongText => "TEXT",
            ColumnType::Int => "INTEGER",
            ColumnType::BigInt => "BIGINT",
            ColumnType::Float => "DOUBLE PRECISION",
            ColumnType::Bool => "BOOLEAN",
            ColumnType::DateTime => "TIMESTAMPTZ",
            ColumnType::NaiveDateTime => "TIMESTAMP",
            ColumnType::Json => "JSONB",
            ColumnType::Binary => "BYTEA",
        }
    }
}
