use super::query_builder::QueryBuilder;

impl<T> QueryBuilder<T> {
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.orders
            .push((column.to_string(), direction.to_string()));
        self
    }

    pub fn order_by_asc(self, column: &str) -> Self {
        self.order_by(column, "ASC")
    }

    pub fn order_by_desc(self, column: &str) -> Self {
        self.order_by(column, "DESC")
    }
}
