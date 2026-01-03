use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Value {
    Null,
    Text(String),
    Int(i64),
    Float(ordered_float::OrderedFloat<f64>),
    Bool(bool),
    DateTime(DateTime<Utc>),
    Json(serde_json::Value),
    Binary(Vec<u8>),
    NaiveDateTime(NaiveDateTime),
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => 0.hash(state),
            Value::Text(s) => s.hash(state),
            Value::Int(i) => i.hash(state),
            Value::Float(f) => f.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::DateTime(dt) => dt.hash(state),
            Value::Json(j) => j.to_string().hash(state),
            Value::Binary(b) => b.hash(state),
            Value::NaiveDateTime(ndt) => ndt.hash(state),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Text(s) => write!(f, "'{}'", s.replace("'", "''")),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", if *b { 1 } else { 0 }),
            Value::DateTime(dt) => write!(f, "'{}'", dt.to_rfc3339()),
            Value::Json(j) => write!(f, "'{}'", j.to_string().replace("'", "''")),
            Value::Binary(b) => write!(f, "X'{}'", hex::encode(b)),
            Value::NaiveDateTime(ndt) => write!(f, "'{}'", ndt.format("%Y-%m-%d %H:%M:%S")),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Int(i as i64)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(ordered_float::OrderedFloat(f))
    }
}

impl From<DateTime<Utc>> for Value {
    fn from(dt: DateTime<Utc>) -> Self {
        Value::DateTime(dt)
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Binary(v)
    }
}

impl From<NaiveDateTime> for Value {
    fn from(ndt: NaiveDateTime) -> Self {
        Value::NaiveDateTime(ndt)
    }
}

pub trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self, String>;
}

impl FromValue for String {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Text(s) => Ok(s),
            _ => Err("Expected Text".to_string()),
        }
    }
}

impl FromValue for i64 {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Int(i) => Ok(i),
            _ => Err("Expected Int".to_string()),
        }
    }
}

impl FromValue for bool {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Bool(b) => Ok(b),
            _ => Err("Expected Bool".to_string()),
        }
    }
}

impl FromValue for f64 {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Float(f) => Ok(f.into_inner()),
            _ => Err("Expected Float".to_string()),
        }
    }
}

impl FromValue for u32 {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Int(i) => {
                if i < 0 {
                    Err("Int value is negative, cannot convert to u32".to_string())
                } else if i > u32::MAX as i64 {
                    Err("Int value out of range for u32".to_string())
                } else {
                    Ok(i as u32)
                }
            }
            _ => Err("Expected Int".to_string()),
        }
    }
}

impl FromValue for u64 {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Int(i) => {
                if i >= 0 {
                    Ok(i as u64)
                } else {
                    Err("Int value is negative, cannot convert to u64".to_string())
                }
            }
            _ => Err("Expected Int".to_string()),
        }
    }
}

impl FromValue for i32 {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Int(i) => {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    Ok(i as i32)
                } else {
                    Err("Int value out of range for i32".to_string())
                }
            }
            _ => Err("Expected Int".to_string()),
        }
    }
}

impl FromValue for f32 {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Float(f) => Ok(f.into_inner() as f32),
            _ => Err("Expected Float".to_string()),
        }
    }
}

impl FromValue for NaiveDateTime {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::NaiveDateTime(ndt) => Ok(ndt),
            _ => Err("Expected NaiveDateTime".to_string()),
        }
    }
}

impl FromValue for serde_json::Value {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Json(j) => Ok(j),
            _ => Err("Expected Json".to_string()),
        }
    }
}

impl FromValue for Vec<u8> {
    fn from_value(v: Value) -> Result<Self, String> {
        match v {
            Value::Binary(b) => Ok(b),
            _ => Err("Expected Binary".to_string()),
        }
    }
}
