use super::limit::Limit;

/// Represents the number of items per page for pagination.
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