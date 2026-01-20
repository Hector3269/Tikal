pub mod application;
pub use infrastructure::config;
pub mod domain;
pub mod infrastructure;

use crate::infrastructure::database::factory::DatabaseFactory;

pub async fn init() -> Result<application::TikalApp, Box<dyn std::error::Error>> {
    let config = config::DatabaseConfig::from_env()?;
    let pool = DatabaseFactory::create_pool(&config).await?;

    tracing::info!(
        "Tikal ORM initialized successfully with {} database",
        config.driver
    );

    Ok(application::TikalApp::new(pool))
}

pub mod prelude {
    pub use crate::application::TikalApp;
    pub use crate::domain::model::{ActiveModel, Entity, FromRow, Lazy, Validate};
    pub use crate::domain::query::builder::{Operator, OrderDirection, QueryBuilder};
    pub use crate::domain::repositories::Repository;
    pub use crate::domain::value_objects::{FromValue, Value};
    pub use crate::domain::{TikalError, TikalResult};
    pub use crate::infrastructure::repositories::SqlRepository;
    pub use tikal_macros::*;
}
