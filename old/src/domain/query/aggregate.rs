#[derive(Debug, Clone, PartialEq)]
pub enum AggregateFunction {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct Aggregate {
    pub function: AggregateFunction,
    pub column: Option<String>,
    pub alias: Option<String>,
}

impl Aggregate {
    pub fn count() -> Self {
        Self {
            function: AggregateFunction::Count,
            column: None,
            alias: None,
        }
    }

    pub fn count_column(column: &str) -> Self {
        Self {
            function: AggregateFunction::Count,
            column: Some(column.to_string()),
            alias: None,
        }
    }

    pub fn sum(column: &str) -> Self {
        Self {
            function: AggregateFunction::Sum,
            column: Some(column.to_string()),
            alias: None,
        }
    }

    pub fn avg(column: &str) -> Self {
        Self {
            function: AggregateFunction::Avg,
            column: Some(column.to_string()),
            alias: None,
        }
    }

    pub fn min(column: &str) -> Self {
        Self {
            function: AggregateFunction::Min,
            column: Some(column.to_string()),
            alias: None,
        }
    }

    pub fn max(column: &str) -> Self {
        Self {
            function: AggregateFunction::Max,
            column: Some(column.to_string()),
            alias: None,
        }
    }

    pub fn as_alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }
}
