use crate::kernel::error::KernelError;
use crate::kernel::types::core::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum CastType {
    Bool,
    Int,
    Float,
    String,
    Json,
    DateTime,
}

impl CastType {
    pub fn from_str(s: &str) -> Result<Self, KernelError> {
        match s {
            "bool" => Ok(CastType::Bool),
            "int" => Ok(CastType::Int),
            "float" => Ok(CastType::Float),
            "string" => Ok(CastType::String),
            "json" => Ok(CastType::Json),
            "datetime" => Ok(CastType::DateTime),
            _ => Err(KernelError::mapping(
                "Cast",
                &format!("Unknown cast type: {}", s),
            )),
        }
    }

    pub fn cast_from_value(&self, value: &Value) -> Result<Value, KernelError> {
        match (self, value) {
            (CastType::Bool, Value::Bool(b)) => Ok(Value::Bool(*b)),
            (CastType::Bool, Value::Int(i)) => Ok(Value::Bool(*i != 0)),
            (CastType::Bool, Value::String(s)) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Ok(Value::Bool(true)),
                "false" | "0" | "no" | "off" | "" => Ok(Value::Bool(false)),
                _ => Err(KernelError::mapping("Cast", "Cannot cast string to bool")),
            },
            (CastType::Int, Value::Int(i)) => Ok(Value::Int(*i)),
            (CastType::Int, Value::Float(f)) => Ok(Value::Int(*f as i64)),
            (CastType::Int, Value::String(s)) => s
                .parse::<i64>()
                .map(Value::Int)
                .map_err(|_| KernelError::mapping("Cast", "Cannot cast string to int")),
            (CastType::Float, Value::Float(f)) => Ok(Value::Float(*f)),
            (CastType::Float, Value::Int(i)) => Ok(Value::Float(*i as f64)),
            (CastType::Float, Value::String(s)) => s
                .parse::<f64>()
                .map(Value::Float)
                .map_err(|_| KernelError::mapping("Cast", "Cannot cast string to float")),
            (CastType::String, Value::String(s)) => Ok(Value::String(s.clone())),
            (CastType::String, Value::Int(i)) => Ok(Value::String(i.to_string())),
            (CastType::String, Value::Float(f)) => Ok(Value::String(f.to_string())),
            (CastType::String, Value::Bool(b)) => Ok(Value::String(b.to_string())),
            (CastType::Json, _) => Ok(value.clone()),
            (CastType::DateTime, Value::String(s)) => Ok(Value::String(s.clone())),
            _ => Err(KernelError::mapping(
                "Cast",
                &format!("Cannot cast {:?} to {:?}", value, self),
            )),
        }
    }

    pub fn cast_to_value(&self, value: &Value) -> Result<Value, KernelError> {
        Ok(value.clone())
    }
}
