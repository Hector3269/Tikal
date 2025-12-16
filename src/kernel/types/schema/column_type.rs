/// Represents the data type of a database column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    Integer,
    BigInteger,
    String,
    Text,
    Boolean,
    Float,
    Double,
    Date,
    DateTime,
    Time,
    Binary,
}