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
pub struct QueryStats {
    pub execution_time: std::time::Duration,
    pub rows_returned: usize,
    pub rows_affected: u64,
    pub success: bool,
    pub error_message: Option<String>,
}
