pub use crate::kernel::types::core::*;
pub use crate::kernel::types::db::*;
pub use crate::kernel::types::query::*;
pub use crate::kernel::types::relation::*;
pub use crate::kernel::types::pagination::*;
pub use crate::kernel::types::cache::*;

pub type DbResult<T> = Result<T, crate::kernel::error::KernelError>;
pub type OptionalDbValue<T> = Option<T>;

pub mod query {
    pub use crate::kernel::types::query::*;
}

pub mod relation {
    pub use crate::kernel::types::relation::*;
}

pub mod pagination {
    pub use crate::kernel::types::pagination::*;
}

pub mod cache {
    pub use crate::kernel::types::cache::*;
}

pub mod db {
    pub use crate::kernel::types::db::*;
}


pub mod core {
    pub use crate::kernel::types::core::*;
}