#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DriverName {
    SQLite,
    MySQL,
    PostgreSQL,
}

impl DriverName {
    pub fn as_str(&self) -> &str {
        match self {
            DriverName::SQLite => "sqlite",
            DriverName::MySQL => "mysql",
            DriverName::PostgreSQL => "postgresql",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sqlite" => Some(DriverName::SQLite),
            "mysql" => Some(DriverName::MySQL),
            "postgresql" | "postgres" => Some(DriverName::PostgreSQL),
            _ => None,
        }
    }
}