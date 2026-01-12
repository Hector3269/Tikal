use crate::domain::query::builder::{Operator, OrderDirection};
use crate::domain::value_objects::Value;
use crate::infrastructure::query_builder::ast::*;
use crate::infrastructure::query_builder::generators::SqlGenerator;

pub struct CommonGenerator;

impl CommonGenerator {
    pub fn generate_select<G: SqlGenerator + ?Sized>(
        gen: &G,
        query: &SelectQuery,
    ) -> (String, Vec<Value>) {
        let mut sql = String::from("SELECT ");
        let mut params = Vec::new();

        if query.distinct {
            sql.push_str("DISTINCT ");
        }

        let columns: Vec<String> = query
            .columns
            .iter()
            .map(|e| Self::expression_to_sql(gen, e))
            .collect();
        sql.push_str(&columns.join(", "));

        sql.push_str(" FROM ");
        sql.push_str(&gen.quote_identifier(&query.table));

        for join in &query.joins {
            let join_type = match join.join_type {
                JoinType::Inner => "INNER JOIN",
                JoinType::Left => "LEFT JOIN",
                JoinType::Right => "RIGHT JOIN",
                JoinType::Full => "FULL OUTER JOIN",
            };
            sql.push_str(&format!(
                " {} {} ON ",
                join_type,
                gen.quote_identifier(&join.table)
            ));
            let cond_sql = Self::condition_to_sql(gen, &join.on, &mut params);
            sql.push_str(&cond_sql);
        }

        if !query.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut filter_sqls = Vec::new();
            for filter in &query.filters {
                filter_sqls.push(Self::condition_to_sql(gen, filter, &mut params));
            }
            sql.push_str(&filter_sqls.join(" AND "));
        }

        if !query.group_by.is_empty() {
            sql.push_str(" GROUP BY ");
            let groups: Vec<String> = query
                .group_by
                .iter()
                .map(|e| Self::expression_to_sql(gen, e))
                .collect();
            sql.push_str(&groups.join(", "));
        }

        if !query.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            let orders: Vec<String> = query
                .order_by
                .iter()
                .map(|o| {
                    let dir = match o.direction {
                        OrderDirection::Asc => "ASC",
                        OrderDirection::Desc => "DESC",
                    };
                    format!("{} {}", Self::expression_to_sql(gen, &o.expression), dir)
                })
                .collect();
            sql.push_str(&orders.join(", "));
        }

        if !query.having.is_empty() {
            sql.push_str(" HAVING ");
            let mut filter_sqls = Vec::new();
            for filter in &query.having {
                filter_sqls.push(Self::condition_to_sql(gen, filter, &mut params));
            }
            sql.push_str(&filter_sqls.join(" AND "));
        }

        if let Some(limit) = query.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = query.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, params)
    }

    pub fn generate_insert<G: SqlGenerator + ?Sized>(
        gen: &G,
        query: &InsertQuery,
    ) -> (String, Vec<Value>) {
        let columns: Vec<String> = query
            .columns
            .iter()
            .map(|c| gen.quote_identifier(c))
            .collect();
        let placeholders: Vec<String> = (0..query.columns.len())
            .map(|i| gen.placeholder(i))
            .collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            gen.quote_identifier(&query.table),
            columns.join(", "),
            placeholders.join(", ")
        );

        (sql, query.values.clone())
    }

    pub fn generate_update<G: SqlGenerator + ?Sized>(
        gen: &G,
        query: &UpdateQuery,
    ) -> (String, Vec<Value>) {
        let mut params = Vec::new();
        let mut assignments = Vec::new();

        for (i, (col, val)) in query.assignments.iter().enumerate() {
            assignments.push(format!(
                "{} = {}",
                gen.quote_identifier(col),
                gen.placeholder(i)
            ));
            params.push(val.clone());
        }

        let mut sql = format!(
            "UPDATE {} SET {}",
            gen.quote_identifier(&query.table),
            assignments.join(", ")
        );

        if !query.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut filter_sqls = Vec::new();
            for filter in &query.filters {
                filter_sqls.push(Self::condition_to_sql(gen, filter, &mut params));
            }
            sql.push_str(&filter_sqls.join(" AND "));
        }

        (sql, params)
    }

    pub fn generate_delete<G: SqlGenerator + ?Sized>(
        gen: &G,
        query: &DeleteQuery,
    ) -> (String, Vec<Value>) {
        let mut params = Vec::new();
        let mut sql = format!("DELETE FROM {}", gen.quote_identifier(&query.table));

        if !query.filters.is_empty() {
            sql.push_str(" WHERE ");
            let mut filter_sqls = Vec::new();
            for filter in &query.filters {
                filter_sqls.push(Self::condition_to_sql(gen, filter, &mut params));
            }
            sql.push_str(&filter_sqls.join(" AND "));
        }

        (sql, params)
    }

    fn expression_to_sql<G: SqlGenerator + ?Sized>(gen: &G, expr: &Expression) -> String {
        match expr {
            Expression::Column(col) => {
                if col == "*" {
                    "*".to_string()
                } else {
                    gen.quote_identifier(col)
                }
            }
            Expression::QualifiedColumn(table, col) => {
                format!(
                    "{}.{}",
                    gen.quote_identifier(table),
                    gen.quote_identifier(col)
                )
            }
            Expression::Literal(val) => val.to_string(),
            Expression::Function(name, args) => {
                let arg_sqls: Vec<String> = args
                    .iter()
                    .map(|e| Self::expression_to_sql(gen, e))
                    .collect();
                format!("{}({})", name, arg_sqls.join(", "))
            }
        }
    }

    fn condition_to_sql<G: SqlGenerator + ?Sized>(
        gen: &G,
        cond: &Condition,
        params: &mut Vec<Value>,
    ) -> String {
        let left = Self::expression_to_sql(gen, &cond.left);
        let op = match cond.operator {
            Operator::Eq => "=",
            Operator::Ne => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
            Operator::Like => "LIKE",
            Operator::In => "IN",
        };

        if cond.operator == Operator::In {
            let mut placeholders = Vec::new();
            for expr in &cond.right {
                if let Expression::Literal(val) = expr {
                    placeholders.push(gen.placeholder(params.len()));
                    params.push(val.clone());
                }
            }
            format!("{} IN ({})", left, placeholders.join(", "))
        } else {
            let right = if let Some(expr) = cond.right.get(0) {
                match expr {
                    Expression::Literal(val) => {
                        params.push(val.clone());
                        gen.placeholder(params.len() - 1)
                    }
                    _ => Self::expression_to_sql(gen, expr),
                }
            } else {
                "NULL".to_string()
            };
            format!("{} {} {}", left, op, right)
        }
    }
}
