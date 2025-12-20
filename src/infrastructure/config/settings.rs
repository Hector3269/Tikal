use crate::kernel::types::core::non_empty_string::NonEmptyString;
use crate::kernel::types::db::driver_name::DriverName;
use crate::kernel::error::KernelError;
use super::env::get_env_var;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: NonEmptyString,
    pub driver: DriverName,
    pub pool_size: u32,
}

impl Settings {
    pub fn new() -> Result<Self, KernelError> {
        let database_url = get_env_var("DATABASE_URL")
            .and_then(|s| NonEmptyString::new(s))
            .ok_or_else(|| KernelError::config("DATABASE_URL is required and must be non-empty"))?;

        let driver_name = get_env_var("DATABASE_DRIVER")
            .ok_or_else(|| KernelError::config("DATABASE_DRIVER is required"))?;

        let driver = DriverName::from_str(&driver_name)
            .ok_or_else(|| KernelError::config("DATABASE_DRIVER must be one of: sqlite, mysql, postgresql"))?;

        let pool_size = get_env_var("DATABASE_POOL_SIZE")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(10);

        Ok(Settings {
            database_url,
            driver,
            pool_size,
        })
    }
}