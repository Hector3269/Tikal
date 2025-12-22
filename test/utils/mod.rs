use tikal::kernel::types::core::Value;

#[allow(dead_code)]
pub fn assert_sql(actual: &str, expected: &str) {
    assert_eq!(
        actual.replace('\n', "").replace("  ", " "),
        expected.replace('\n', "").replace("  ", " ")
    );
}

#[allow(dead_code)]
pub fn values<I: Into<Value>>(vals: Vec<I>) -> Vec<Value> {
    vals.into_iter().map(|v| v.into()).collect()
}
