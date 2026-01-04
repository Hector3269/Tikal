#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub driver_type: DriverType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DriverType {
    PostgreSQL,
    MySQL,
    SQLite,
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: std::time::Duration,
    pub idle_timeout: Option<std::time::Duration>,
    pub max_lifetime: Option<std::time::Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 1,
            connect_timeout: std::time::Duration::from_secs(30),
            idle_timeout: Some(std::time::Duration::from_secs(600)),
            max_lifetime: Some(std::time::Duration::from_secs(1800)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryStats {
    pub execution_time: std::time::Duration,
    pub rows_returned: usize,
    pub rows_affected: u64,
    pub success: bool,
    pub error_message: Option<String>,
}