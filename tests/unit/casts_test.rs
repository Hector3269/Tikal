use std::collections::HashMap;
use tikal::domain::casts::{CastType, Casts, HasCasts};
use tikal::kernel::error::KernelError;
use tikal::kernel::types::core::Value;

#[test]
fn cast_type_from_str_valid() {
    assert_eq!("bool".parse::<CastType>().unwrap(), CastType::Bool);
    assert_eq!("int".parse::<CastType>().unwrap(), CastType::Int);
    assert_eq!("float".parse::<CastType>().unwrap(), CastType::Float);
    assert_eq!("string".parse::<CastType>().unwrap(), CastType::String);
    assert_eq!("json".parse::<CastType>().unwrap(), CastType::Json);
    assert_eq!("datetime".parse::<CastType>().unwrap(), CastType::DateTime);
}

#[test]
fn cast_type_from_str_invalid() {
    let result = "invalid".parse::<CastType>();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        KernelError::MappingError { .. }
    ));
}

#[test]
fn cast_type_cast_from_value_bool() {
    // Bool from Bool
    assert_eq!(
        CastType::Bool.cast_from_value(&Value::Bool(true)).unwrap(),
        Value::Bool(true)
    );

    // Bool from Int
    assert_eq!(
        CastType::Bool.cast_from_value(&Value::Int(1)).unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        CastType::Bool.cast_from_value(&Value::Int(0)).unwrap(),
        Value::Bool(false)
    );

    // Bool from String
    assert_eq!(
        CastType::Bool
            .cast_from_value(&Value::String("true".to_string()))
            .unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        CastType::Bool
            .cast_from_value(&Value::String("false".to_string()))
            .unwrap(),
        Value::Bool(false)
    );
    assert_eq!(
        CastType::Bool
            .cast_from_value(&Value::String("1".to_string()))
            .unwrap(),
        Value::Bool(true)
    );
    assert_eq!(
        CastType::Bool
            .cast_from_value(&Value::String("0".to_string()))
            .unwrap(),
        Value::Bool(false)
    );

    // Invalid bool string
    let result = CastType::Bool.cast_from_value(&Value::String("invalid".to_string()));
    assert!(result.is_err());
}

#[test]
fn cast_type_cast_from_value_int() {
    // Int from Int
    assert_eq!(
        CastType::Int.cast_from_value(&Value::Int(42)).unwrap(),
        Value::Int(42)
    );

    // Int from Float
    assert_eq!(
        CastType::Int.cast_from_value(&Value::Float(42.7)).unwrap(),
        Value::Int(42)
    );

    // Int from String
    assert_eq!(
        CastType::Int
            .cast_from_value(&Value::String("42".to_string()))
            .unwrap(),
        Value::Int(42)
    );

    // Invalid int string
    let result = CastType::Int.cast_from_value(&Value::String("invalid".to_string()));
    assert!(result.is_err());
}

#[test]
fn cast_type_cast_from_value_float() {
    // Float from Float
    assert_eq!(
        CastType::Float
            .cast_from_value(&Value::Float(42.5))
            .unwrap(),
        Value::Float(42.5)
    );

    // Float from Int
    assert_eq!(
        CastType::Float.cast_from_value(&Value::Int(42)).unwrap(),
        Value::Float(42.0)
    );

    // Float from String
    assert_eq!(
        CastType::Float
            .cast_from_value(&Value::String("42.5".to_string()))
            .unwrap(),
        Value::Float(42.5)
    );

    // Invalid float string
    let result = CastType::Float.cast_from_value(&Value::String("invalid".to_string()));
    assert!(result.is_err());
}

#[test]
fn cast_type_cast_from_value_string() {
    // String from String
    assert_eq!(
        CastType::String
            .cast_from_value(&Value::String("hello".to_string()))
            .unwrap(),
        Value::String("hello".to_string())
    );

    // String from Int
    assert_eq!(
        CastType::String.cast_from_value(&Value::Int(42)).unwrap(),
        Value::String("42".to_string())
    );

    // String from Float
    assert_eq!(
        CastType::String
            .cast_from_value(&Value::Float(42.5))
            .unwrap(),
        Value::String("42.5".to_string())
    );

    // String from Bool
    assert_eq!(
        CastType::String
            .cast_from_value(&Value::Bool(true))
            .unwrap(),
        Value::String("true".to_string())
    );
}

#[test]
fn cast_type_cast_from_value_json() {
    // Json accepts any value
    assert_eq!(
        CastType::Json
            .cast_from_value(&Value::String("data".to_string()))
            .unwrap(),
        Value::String("data".to_string())
    );
    assert_eq!(
        CastType::Json.cast_from_value(&Value::Int(42)).unwrap(),
        Value::Int(42)
    );
}

#[test]
fn cast_type_cast_from_value_datetime() {
    // DateTime from String
    assert_eq!(
        CastType::DateTime
            .cast_from_value(&Value::String("2023-01-01".to_string()))
            .unwrap(),
        Value::String("2023-01-01".to_string())
    );
}

#[test]
fn cast_type_cast_from_value_invalid() {
    // Invalid cast: Bool from Float
    let result = CastType::Bool.cast_from_value(&Value::Float(1.0));
    assert!(result.is_err());

    // Invalid cast: Int from Bool
    let result = CastType::Int.cast_from_value(&Value::Bool(true));
    assert!(result.is_err());
}

#[test]
fn cast_type_cast_to_value() {
    // cast_to_value just returns the value as-is for now
    assert_eq!(
        CastType::Bool.cast_to_value(&Value::Bool(true)).unwrap(),
        Value::Bool(true)
    );
}

// Tests for casts_struct.rs
#[test]
fn casts_new() {
    let casts = Casts::new();
    assert!(casts.definitions.is_empty());
}

#[test]
fn casts_add() {
    let casts = Casts::new()
        .add("field1", CastType::Int)
        .add("field2", CastType::String);
    assert_eq!(casts.definitions.len(), 2);
    assert_eq!(casts.definitions.get("field1"), Some(&CastType::Int));
    assert_eq!(casts.definitions.get("field2"), Some(&CastType::String));
}

#[test]
fn casts_add_from_str() {
    let casts = Casts::new()
        .add_from_str("field1", "int")
        .unwrap()
        .add_from_str("field2", "string")
        .unwrap();
    assert_eq!(casts.definitions.len(), 2);
    assert_eq!(casts.definitions.get("field1"), Some(&CastType::Int));
    assert_eq!(casts.definitions.get("field2"), Some(&CastType::String));
}

#[test]
fn casts_add_from_str_invalid() {
    let result = Casts::new().add_from_str("field", "invalid");
    assert!(result.is_err());
}

#[test]
fn casts_get_cast() {
    let casts = Casts::new().add("field1", CastType::Int);
    assert_eq!(casts.get_cast("field1"), Some(&CastType::Int));
    assert_eq!(casts.get_cast("nonexistent"), None);
}

#[test]
fn casts_cast_on_load() {
    let casts = Casts::new()
        .add("int_field", CastType::Int)
        .add("str_field", CastType::String);

    // Cast int field
    assert_eq!(
        casts
            .cast_on_load("int_field", &Value::String("42".to_string()))
            .unwrap(),
        Value::Int(42)
    );

    // Cast string field
    assert_eq!(
        casts.cast_on_load("str_field", &Value::Int(42)).unwrap(),
        Value::String("42".to_string())
    );

    // No cast for undefined field
    assert_eq!(
        casts
            .cast_on_load("undefined", &Value::String("value".to_string()))
            .unwrap(),
        Value::String("value".to_string())
    );
}

#[test]
fn casts_cast_on_save() {
    let casts = Casts::new().add("field1", CastType::Int);

    // cast_on_save currently just returns the value
    assert_eq!(
        casts.cast_on_save("field1", &Value::Int(42)).unwrap(),
        Value::Int(42)
    );

    // No cast for undefined field
    assert_eq!(
        casts
            .cast_on_save("undefined", &Value::String("value".to_string()))
            .unwrap(),
        Value::String("value".to_string())
    );
}

#[test]
fn casts_default() {
    let casts = Casts::default();
    assert!(casts.definitions.is_empty());
}

// Tests for HasCasts trait
struct TestModel;

impl HasCasts for TestModel {
    fn casts() -> Casts {
        Casts::new()
            .add("id", CastType::Int)
            .add("name", CastType::String)
    }
}

#[test]
fn has_casts_cast_attributes_on_load() {
    let mut attributes = HashMap::from([
        ("id".to_string(), Value::String("123".to_string())),
        ("name".to_string(), Value::String("test".to_string())),
        ("untyped".to_string(), Value::String("value".to_string())),
    ]);

    TestModel::cast_attributes_on_load(&mut attributes).unwrap();

    assert_eq!(attributes.get("id"), Some(&Value::Int(123)));
    assert_eq!(
        attributes.get("name"),
        Some(&Value::String("test".to_string()))
    );
    assert_eq!(
        attributes.get("untyped"),
        Some(&Value::String("value".to_string()))
    );
}

#[test]
fn has_casts_cast_attributes_on_save() {
    let mut attributes = HashMap::from([
        ("id".to_string(), Value::Int(123)),
        ("name".to_string(), Value::String("test".to_string())),
        ("untyped".to_string(), Value::String("value".to_string())),
    ]);

    TestModel::cast_attributes_on_save(&mut attributes).unwrap();

    // cast_on_save currently just returns values as-is
    assert_eq!(attributes.get("id"), Some(&Value::Int(123)));
    assert_eq!(
        attributes.get("name"),
        Some(&Value::String("test".to_string()))
    );
    assert_eq!(
        attributes.get("untyped"),
        Some(&Value::String("value".to_string()))
    );
}
