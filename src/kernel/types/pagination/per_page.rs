use crate::kernel::types::query::limit::Limit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerPage(Limit);

impl PerPage {
    pub fn new(limit: Limit) -> Self {
        Self(limit)
    }

    pub fn limit(&self) -> &Limit {
        &self.0
    }
}