pub mod batch_size;
pub mod page;
pub mod per_page;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    page: page::Page,
    per_page: per_page::PerPage,
    total: u64,
}

impl Pagination {
    pub fn new(page: page::Page, per_page: per_page::PerPage, total: u64) -> Self {
        Self {
            page,
            per_page,
            total,
        }
    }

    pub fn page(&self) -> &page::Page {
        &self.page
    }

    pub fn per_page(&self) -> &per_page::PerPage {
        &self.per_page
    }

    pub fn total(&self) -> u64 {
        self.total
    }
}

pub use batch_size::BatchSize;
pub use page::Page;
pub use per_page::PerPage;
