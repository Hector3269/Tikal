use tikal::domain::queries::{Operator, WhereCondition};
use tikal::kernel::types::core::Value;

#[test]
fn operator_as_sql() {
    assert_eq!(Operator::Eq.as_sql(), "=");
    assert_eq!(Operator::Ne.as_sql(), "!=");
    assert_eq!(Operator::Gt.as_sql(), ">");
    assert_eq!(Operator::Gte.as_sql(), ">=");
    assert_eq!(Operator::Lt.as_sql(), "<");
    assert_eq!(Operator::Lte.as_sql(), "<=");
    assert_eq!(Operator::Like.as_sql(), "LIKE");
    assert_eq!(Operator::In.as_sql(), "IN");
    assert_eq!(Operator::NotIn.as_sql(), "NOT IN");
    assert_eq!(Operator::IsNull.as_sql(), "IS NULL");
    assert_eq!(Operator::IsNotNull.as_sql(), "IS NOT NULL");
}

#[test]
fn operator_supports_multiple() {
    assert!(!Operator::Eq.supports_multiple());
    assert!(!Operator::Ne.supports_multiple());
    assert!(!Operator::Gt.supports_multiple());
    assert!(!Operator::Gte.supports_multiple());
    assert!(!Operator::Lt.supports_multiple());
    assert!(!Operator::Lte.supports_multiple());
    assert!(!Operator::Like.supports_multiple());
    assert!(Operator::In.supports_multiple());
    assert!(Operator::NotIn.supports_multiple());
    assert!(!Operator::IsNull.supports_multiple());
    assert!(!Operator::IsNotNull.supports_multiple());
}

// Tests for where_condition.rs
#[test]
fn where_condition_simple() {
    let wc = WhereCondition::simple("id", Operator::Eq, Value::Int(1));
    assert_eq!(wc.column, "id");
    assert_eq!(wc.operator, Operator::Eq);
    assert_eq!(wc.value, Some(Value::Int(1)));
    assert_eq!(wc.values, None);
}

#[test]
fn where_condition_multi() {
    let wc = WhereCondition::multi(
        "id",
        Operator::In,
        vec![Value::Int(1), Value::Int(2), Value::Int(3)],
    );
    assert_eq!(wc.column, "id");
    assert_eq!(wc.operator, Operator::In);
    assert_eq!(wc.value, None);
    assert_eq!(
        wc.values,
        Some(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
    );
}

#[test]
#[should_panic(expected = "Operator does not support multiple values")]
fn where_condition_multi_panics_for_unsupported_operator() {
    WhereCondition::multi(
        "id",
        Operator::Eq,
        vec![Value::Int(1), Value::Int(2), Value::Int(3)],
    );
}

#[test]
fn where_condition_null() {
    let wc = WhereCondition::null("deleted_at", Operator::IsNull);
    assert_eq!(wc.column, "deleted_at");
    assert_eq!(wc.operator, Operator::IsNull);
    assert_eq!(wc.value, None);
    assert_eq!(wc.values, None);
}
