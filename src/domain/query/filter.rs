use crate::domain::query::builder::Operator;
use crate::domain::value_objects::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    pub column: String,
    pub operator: Operator,
    pub values: Vec<Value>,
}

impl Filter {
    pub fn new(column: String, operator: Operator, values: Vec<Value>) -> Self {
        Self {
            column,
            operator,
            values,
        }
    }

    pub fn eq(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Eq, vec![value.into()])
    }

    pub fn ne(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Ne, vec![value.into()])
    }

    pub fn gt(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Gt, vec![value.into()])
    }

    pub fn lt(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Lt, vec![value.into()])
    }

    pub fn gte(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Gte, vec![value.into()])
    }

    pub fn lte(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Lte, vec![value.into()])
    }

    pub fn like(column: &str, value: impl Into<Value>) -> Self {
        Self::new(column.to_string(), Operator::Like, vec![value.into()])
    }

    pub fn in_values(column: &str, values: Vec<impl Into<Value>>) -> Self {
        Self::new(
            column.to_string(),
            Operator::In,
            values.into_iter().map(|v| v.into()).collect(),
        )
    }

    pub fn to_sql(&self, placeholder_generator: impl Fn(usize) -> String) -> (String, Vec<Value>) {
        let params = self.values.clone();

        match self.operator {
            Operator::Eq => (
                format!("{} = {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::Ne => (
                format!("{} != {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::Gt => (
                format!("{} > {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::Lt => (
                format!("{} < {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::Gte => (
                format!("{} >= {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::Lte => (
                format!("{} <= {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::Like => (
                format!("{} LIKE {}", self.column, placeholder_generator(0)),
                params,
            ),
            Operator::In => {
                let placeholders: Vec<String> = (0..params.len())
                    .map(|i| placeholder_generator(i))
                    .collect();
                (
                    format!("{} IN ({})", self.column, placeholders.join(", ")),
                    params,
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterBuilder {
    filters: Vec<Filter>,
    logic: FilterLogic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterLogic {
    And,
    Or,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            logic: FilterLogic::And,
        }
    }

    pub fn and(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self.logic = FilterLogic::And;
        self
    }

    pub fn or(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self.logic = FilterLogic::Or;
        self
    }

    pub fn add_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn build(self) -> FilterGroup {
        FilterGroup {
            filters: self.filters,
            logic: self.logic,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterGroup {
    filters: Vec<Filter>,
    logic: FilterLogic,
}

impl FilterGroup {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            logic: FilterLogic::And,
        }
    }

    pub fn and(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self.logic = FilterLogic::And;
        self
    }

    pub fn or(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self.logic = FilterLogic::Or;
        self
    }

    pub fn add_filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    pub fn len(&self) -> usize {
        self.filters.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Filter> {
        self.filters.iter()
    }

    pub fn to_sql(&self, placeholder_generator: impl Fn(usize) -> String) -> (String, Vec<Value>) {
        if self.filters.is_empty() {
            return (String::new(), Vec::new());
        }

        if self.filters.len() == 1 {
            return self.filters[0].to_sql(placeholder_generator);
        }

        let mut all_params = Vec::new();
        let mut conditions = Vec::new();
        let mut param_offset = 0;

        for filter in &self.filters {
            let (condition, mut params) =
                filter.to_sql(&|i| placeholder_generator(i + param_offset));
            conditions.push(condition);
            all_params.append(&mut params);
            param_offset += params.len();
        }

        let operator = match self.logic {
            FilterLogic::And => "AND",
            FilterLogic::Or => "OR",
        };

        let sql = format!("({})", conditions.join(&format!(" {} ", operator)));
        (sql, all_params)
    }
}

impl Default for FilterGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub fn column_eq(column: &str, value: impl Into<Value>) -> Filter {
    Filter::eq(column, value)
}

pub fn column_in(column: &str, values: Vec<impl Into<Value>>) -> Filter {
    Filter::in_values(column, values)
}

pub fn column_like(column: &str, pattern: impl Into<Value>) -> Filter {
    Filter::like(column, pattern)
}
