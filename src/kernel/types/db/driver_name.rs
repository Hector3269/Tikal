use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DriverName {
    SQLite,
    MySQL,
    PostgreSQL,
}

impl FromStr for DriverName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sqlite" => Ok(DriverName::SQLite),
            "mysql" => Ok(DriverName::MySQL),
            "postgresql" | "postgres" => Ok(DriverName::PostgreSQL),
            _ => Err(()),
        }
    }
}

impl DriverName {
    pub fn as_str(&self) -> &str {
        match self {
            DriverName::SQLite => "sqlite",
            DriverName::MySQL => "mysql",
            DriverName::PostgreSQL => "postgresql",
        }
    }
}
