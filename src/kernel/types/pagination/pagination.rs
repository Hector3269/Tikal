use super::page::Page;
use super::per_page::PerPage;

/// Represents pagination information for query results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    page: Page,
    per_page: PerPage,
    total: u64,
}

impl Pagination {
    pub fn new(page: Page, per_page: PerPage, total: u64) -> Self {
        Self {
            page,
            per_page,
            total,
        }
    }

    pub fn page(&self) -> &Page {
        &self.page
    }

    pub fn per_page(&self) -> &PerPage {
        &self.per_page
    }

    pub fn total(&self) -> u64 {
        self.total
    }
}