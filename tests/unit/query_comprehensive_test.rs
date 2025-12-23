use tikal::domain::queries::{Operator, WhereCondition};
use tikal::kernel::types::core::Value;

#[test]
fn test_where_condition_creation() {
    let simple = WhereCondition::simple("id".to_string(), Operator::Eq, Value::Int(1));
    assert_eq!(simple.column, "id");
    assert_eq!(simple.operator, Operator::Eq);
    assert_eq!(simple.value, Some(Value::Int(1)));
    assert_eq!(simple.values, None);

    let multi = WhereCondition::multi(
        "ids".to_string(),
        Operator::In,
        vec![Value::Int(1), Value::Int(2)],
    );
    assert_eq!(multi.column, "ids");
    assert_eq!(multi.operator, Operator::In);
    assert_eq!(multi.value, None);
    assert_eq!(multi.values, Some(vec![Value::Int(1), Value::Int(2)]));

    let null_cond = WhereCondition::null("deleted_at".to_string(), Operator::IsNull);
    assert_eq!(null_cond.column, "deleted_at");
    assert_eq!(null_cond.operator, Operator::IsNull);
    assert_eq!(null_cond.value, None);
    assert_eq!(null_cond.values, None);
}

#[test]
fn test_operator_properties() {
    // Test all operators return valid SQL
    let operators = vec![
        Operator::Eq, Operator::Ne, Operator::Gt, Operator::Gte,
        Operator::Lt, Operator::Lte, Operator::Like, Operator::In,
        Operator::NotIn, Operator::IsNull, Operator::IsNotNull
    ];

    for op in operators {
        let sql = op.as_sql();
        assert!(!sql.is_empty());
        assert!(!sql.contains("Operator::"));
    }

    // Test supports_multiple
    assert!(Operator::In.supports_multiple());
    assert!(Operator::NotIn.supports_multiple());
    assert!(!Operator::Eq.supports_multiple());
    assert!(!Operator::Like.supports_multiple());
}

#[test]
fn test_where_condition_panics_on_invalid_multi() {
    // This should panic because Eq doesn't support multiple values
    let result = std::panic::catch_unwind(|| {
        WhereCondition::multi("test".to_string(), Operator::Eq, vec![Value::Int(1), Value::Int(2)]);
    });
    assert!(result.is_err());
}

#[test]
fn test_value_enum_variants() {
    // Test that all Value variants work
    assert_eq!(Value::Int(42), Value::Int(42));
    assert_eq!(Value::Float(3.14), Value::Float(3.14));
    assert_eq!(Value::Bool(true), Value::Bool(true));
    assert_eq!(Value::String("test".to_string()), Value::String("test".to_string()));
    assert_eq!(Value::Null, Value::Null);
}

#[test]
fn test_value_display_formatting() {
    // Test Display trait implementation
    assert_eq!(format!("{}", Value::Int(123)), "123");
    assert_eq!(format!("{}", Value::Float(45.67)), "45.67");
    assert_eq!(format!("{}", Value::Bool(true)), "true");
    assert_eq!(format!("{}", Value::Bool(false)), "false");
    assert_eq!(format!("{}", Value::String("hello".to_string())), "hello");
    assert_eq!(format!("{}", Value::Null), "NULL");
}

#[test]
fn test_operator_consistency() {
    // All operators should have non-empty SQL representations
    let all_operators = vec![
        Operator::Eq, Operator::Ne, Operator::Gt, Operator::Gte,
        Operator::Lt, Operator::Lte, Operator::Like, Operator::In,
        Operator::NotIn, Operator::IsNull, Operator::IsNotNull
    ];

    for op in all_operators {
        let sql = op.as_sql();
        assert!(!sql.is_empty(), "Operator {:?} has empty SQL", op);
        assert!(!sql.contains("Operator::"), "Operator {:?} contains raw enum name", op);
    }
}

#[test]
fn test_operator_supports_multiple_correctness() {
    // Only IN and NOT IN should support multiple values
    assert!(Operator::In.supports_multiple());
    assert!(Operator::NotIn.supports_multiple());

    // All others should not
    let single_value_ops = vec![
        Operator::Eq, Operator::Ne, Operator::Gt, Operator::Gte,
        Operator::Lt, Operator::Lte, Operator::Like, Operator::IsNull, Operator::IsNotNull
    ];

    for op in single_value_ops {
        assert!(!op.supports_multiple(), "Operator {:?} should not support multiple values", op);
    }
}
