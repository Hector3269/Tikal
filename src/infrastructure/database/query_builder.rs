use std::marker::PhantomData;
use crate::kernel::types::query::{WhereClause, Operator, OrderBy, SortDirection, Limit, Offset};
use crate::kernel::types::schema::ColumnName;
use crate::kernel::types::core::Value;

pub struct QueryBuilder<T> {
    table: String,
    wheres: Vec<WhereClause>,
    order_by: Option<OrderBy>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    _phantom: PhantomData<T>,
}

impl<T> QueryBuilder<T> {
    pub fn new(table: String) -> Self {
        Self {
            table,
            wheres: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
            _phantom: PhantomData,
        }
    }

    pub fn where_clause(mut self, column: &str, operator: Operator, value: impl Into<Value>) -> Self {
        let column_name = ColumnName::new(column);
        self.wheres.push(WhereClause::new(column_name, operator, value.into()));
        self
    }

    pub fn where_eq(self, column: &str, value: impl Into<Value>) -> Self {
        self.where_clause(column, Operator::Eq, value)
    }

    pub fn order_by(mut self, column: &str, direction: SortDirection) -> Self {
        let column_name = ColumnName::new(column);
        self.order_by = Some(OrderBy::new(column_name, direction));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(Limit::new(limit as u32).unwrap());
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(Offset::new(offset as u32));
        self
    }

  
    pub fn to_sql(&self) -> String {
        let mut sql = format!("SELECT * FROM {}", self.table);

        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            let where_strs: Vec<String> = self.wheres.iter()
                .map(|w| format!("{} {} ?", w.column().as_str(), w.operator().as_str()))
                .collect();
            sql.push_str(&where_strs.join(" AND "));
        }

        if let Some(order_by) = &self.order_by {
            sql.push_str(&format!(" ORDER BY {} {}", order_by.column().as_str(), order_by.direction().as_str()));
        }

        if let Some(limit) = &self.limit {
            sql.push_str(&format!(" LIMIT {}", limit.value()));
        }

        if let Some(offset) = &self.offset {
            sql.push_str(&format!(" OFFSET {}", offset.value()));
        }

        sql
    }
    pub fn params(&self) -> Vec<&Value> {
        self.wheres.iter().map(|w| w.value()).collect()
    }
}

impl<T> Default for QueryBuilder<T> {
    fn default() -> Self {
        Self::new(String::new())
    }
}