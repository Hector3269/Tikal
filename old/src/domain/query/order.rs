use crate::domain::query::builder::OrderDirection;

#[derive(Debug, Clone, PartialEq)]
pub struct OrderClause {
    pub column: String,
    pub direction: OrderDirection,
}

impl OrderClause {
    pub fn new(column: String, direction: OrderDirection) -> Self {
        Self { column, direction }
    }

    pub fn asc(column: &str) -> Self {
        Self::new(column.to_string(), OrderDirection::Asc)
    }

    pub fn desc(column: &str) -> Self {
        Self::new(column.to_string(), OrderDirection::Desc)
    }

    pub fn to_sql(&self) -> String {
        let direction_str = match self.direction {
            OrderDirection::Asc => "ASC",
            OrderDirection::Desc => "DESC",
        };
        format!("{} {}", self.column, direction_str)
    }
}

#[derive(Debug, Clone)]
pub struct OrderBuilder {
    orders: Vec<OrderClause>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn then_asc(mut self, column: &str) -> Self {
        self.orders.push(OrderClause::asc(column));
        self
    }

    pub fn then_desc(mut self, column: &str) -> Self {
        self.orders.push(OrderClause::desc(column));
        self
    }

    pub fn add_order(mut self, order: OrderClause) -> Self {
        self.orders.push(order);
        self
    }

    pub fn build(self) -> OrderGroup {
        OrderGroup {
            orders: self.orders,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderGroup {
    orders: Vec<OrderClause>,
}

impl OrderGroup {
    pub fn new() -> Self {
        Self { orders: Vec::new() }
    }

    pub fn add_asc(mut self, column: &str) -> Self {
        self.orders.push(OrderClause::asc(column));
        self
    }

    pub fn add_desc(mut self, column: &str) -> Self {
        self.orders.push(OrderClause::desc(column));
        self
    }

    pub fn add_order(mut self, order: OrderClause) -> Self {
        self.orders.push(order);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn len(&self) -> usize {
        self.orders.len()
    }

    pub fn clear(&mut self) {
        self.orders.clear();
    }

    pub fn to_sql(&self) -> String {
        if self.orders.is_empty() {
            return String::new();
        }

        let order_clauses: Vec<String> = self.orders.iter().map(|order| order.to_sql()).collect();

        order_clauses.join(", ")
    }

    pub fn iter(&self) -> impl Iterator<Item = &OrderClause> {
        self.orders.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut OrderClause> {
        self.orders.iter_mut()
    }
}

impl Default for OrderGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub fn order_asc(column: &str) -> OrderClause {
    OrderClause::asc(column)
}

pub fn order_desc(column: &str) -> OrderClause {
    OrderClause::desc(column)
}

pub fn order() -> OrderBuilder {
    OrderBuilder::new()
}

pub fn order_by(column: &str, direction: OrderDirection) -> OrderGroup {
    OrderGroup::new().add_order(OrderClause::new(column.to_string(), direction))
}

pub fn order_by_asc(column: &str) -> OrderGroup {
    OrderGroup::new().add_asc(column)
}

pub fn order_by_desc(column: &str) -> OrderGroup {
    OrderGroup::new().add_desc(column)
}

pub fn order_by_multiple(columns: &[(&str, OrderDirection)]) -> OrderGroup {
    let mut group = OrderGroup::new();
    for (column, direction) in columns {
        group = group.add_order(OrderClause::new(column.to_string(), direction.clone()));
    }
    group
}

pub mod patterns {
    use super::*;

    pub fn by_created_at_asc() -> OrderGroup {
        order_by_asc("created_at")
    }

    pub fn by_created_at_desc() -> OrderGroup {
        order_by_desc("created_at")
    }

    pub fn by_updated_at_asc() -> OrderGroup {
        order_by_asc("updated_at")
    }

    pub fn by_updated_at_desc() -> OrderGroup {
        order_by_desc("updated_at")
    }

    pub fn by_id_asc() -> OrderGroup {
        order_by_asc("id")
    }

    pub fn by_id_desc() -> OrderGroup {
        order_by_desc("id")
    }

    pub fn by_name_asc() -> OrderGroup {
        order_by_asc("name")
    }

    pub fn by_name_desc() -> OrderGroup {
        order_by_desc("name")
    }
}
